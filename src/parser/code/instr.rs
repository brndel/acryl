use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::{parser::{
    lexer::{Keyword, Token},
    Spanned,
}, ast::{Expr, Instr, Op}};

use super::expr::expr_parser;


fn let_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, Spanned<&'src str>),
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Instr<'src>) {
    just(Token::Keyword(Keyword::Let))
        .ignore_then(ident_raw.clone())
        .then(just(Token::Ctrl(':')).ignore_then(ident_raw).or_not())
        .then_ignore(just(Token::Op(Op::SET)))
        .then(expr)
        .map(|((name, ty), value)| Instr::Let { name, ty, value })
}

fn return_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Instr<'src>) {
    just(Token::Keyword(Keyword::Return))
        .ignore_then(expr.clone())
        .map(|expr| Instr::Return(expr))
}

fn set_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, Spanned<&'src str>),
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Instr<'src>) {
    ident_raw
        .then_ignore(just(Token::Op(Op::SET)))
        .then(expr.clone())
        .map(|(name, value)| Instr::Set { name, value })
}

fn if_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
    block: parser!('tokens, 'src, Spanned<Instr<'src>>),
) -> parser!('tokens, 'src, Spanned<Instr<'src>>) {
    just(Token::Keyword(Keyword::If))
        .ignore_then(expr.clone())
        .then(block.clone())
        .then(
            just(Token::Keyword(Keyword::Else))
                .ignore_then(block)
                .or_not(),
        )
        .map_with_span(|((condition, body), r#else), span| {
            (
                Instr::If {
                    condition,
                    body: Box::new(body),
                    r#else: Box::new(r#else),
                },
                span,
            )
        })
}

fn fn_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, Spanned<&'src str>),
    block: parser!('tokens, 'src, Spanned<Instr<'src>>),
) -> parser!('tokens, 'src, Spanned<Instr<'src>>) {
    just(Token::Keyword(Keyword::Fn))
        .ignore_then(ident_raw.clone())
        .then(
            ident_raw
                .clone()
                .then_ignore(just(Token::Ctrl(':')))
                .then(ident_raw.clone())
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                .map_with_span(|args, span| (args, span)),
        )
        .then(
            just(Token::Op(Op::ARROW))
                .ignore_then(ident_raw.clone())
                .or_not(),
        )
        .then(block)
        .map_with_span(|(((name, args), return_type), body), span| {
            (
                Instr::Fn {
                    name,
                    args,
                    return_type,
                    body: Box::new(body),
                },
                span,
            )
        })
}

pub fn instr_parser<'tokens, 'src: 'tokens>() -> parser!('tokens, 'src, Vec<Spanned<Instr<'src>>>) {
    recursive(|expr| {
        let inline_expr = expr_parser();

        let ident_raw = select! { Token::Word(ident) => ident }.map_with_span(|ident, span| (ident, span));

        let r#let = let_parser(ident_raw.clone(), inline_expr.clone());
        let r#return = return_parser(inline_expr.clone());
        let set = set_parser(ident_raw.clone(), inline_expr.clone());
        let inline = inline_expr.clone().map(|expr| Instr::Expr(expr));

        let line_expr = r#let.or(r#return).or(set).or(inline);

        let line_expr = line_expr
            .then_ignore(just(Token::Ctrl(';')))
            .map_with_span(|line, span| (line, span));

        let block = expr
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            .map_with_span(|block, span| (Instr::Block(block), span));

        let r#if = if_parser(inline_expr.clone(), block.clone());
        let r#fn = fn_parser(ident_raw.clone(), block.clone());

        r#fn.or(r#if)
            .or(block)
            .or(line_expr)
            .repeated()
            .collect::<Vec<_>>()
    })
}
