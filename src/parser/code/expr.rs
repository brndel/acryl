use std::fmt::Display;

use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::parser::{
    content::ContentToken,
    lexer::{Keyword, Op, Token},
    Spanned,
};

use super::inline_expr::{inline_expr_parser, InlineExpr};

#[derive(Debug)]
pub enum Expr<'src> {
    Inline(Spanned<InlineExpr<'src>>),
    Let {
        name: &'src str,
        value: Spanned<InlineExpr<'src>>,
    },
    Set {
        name: &'src str,
        value: Spanned<InlineExpr<'src>>,
    },
    If {
        condition: Spanned<InlineExpr<'src>>,
        body: Box<Spanned<Self>>,
        r#else: Box<Option<Spanned<Self>>>,
    },
    Return(Spanned<InlineExpr<'src>>),

    Fn {
        name: &'src str,
        args: Vec<(&'src str, &'src str)>,
        return_type: Option<&'src str>,
        body: Box<Spanned<Self>>,
    },

    Block(Vec<Spanned<Self>>),
}

impl<'src> Display for Expr<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Inline(expr) => write!(f, "{}", expr.0),
            Expr::Let { name, value } => write!(f, "let {} = {}", name, value.0),
            Expr::Set { name, value } => write!(f, "{} = {}", name, value.0),
            Expr::If {
                condition,
                body: then,
                r#else,
            } => {
                writeln!(f, "if {} {}", condition.0, then.0)?;
                if let Some(r#else) = r#else.as_ref() {
                    writeln!(f, "else {}", r#else.0)
                } else {
                    Ok(())
                }
            }
            Expr::Return(expr) => write!(f, "return {}", expr.0),
            Expr::Fn {
                name,
                args,
                return_type,
                body,
            } => {
                write!(f, "fn {}(", name)?;

                for (name, r#type) in args {
                    write!(f, "{}: {}", name, r#type)?;
                }
                write!(f, ")")?;

                if let Some(return_type) = return_type {
                    write!(f, " -> {} ", return_type)?;
                }

                write!(f, "{}", body.0)
            }
            Expr::Block(expressions) => {
                writeln!(f, "{{")?;
                for expr in expressions {
                    writeln!(f, "{}", expr.0)?;
                }
                write!(f, "}} ")
            }

        }
    }
}

fn let_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, &'src str),
    inline_expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, Expr<'src>) {
    just(Token::Keyword(Keyword::Let))
        .ignore_then(ident_raw)
        .then_ignore(just(Token::Op(Op::Set)))
        .then(inline_expr)
        .map(|(name, value)| Expr::Let { name, value })
}

fn return_parser<'tokens, 'src: 'tokens>(
    inline_expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, Expr<'src>) {
    just(Token::Keyword(Keyword::Return))
        .ignore_then(inline_expr.clone())
        .map(|expr| Expr::Return(expr))
}

fn set_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, &'src str),
    inline_expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, Expr<'src>) {
    ident_raw
        .then_ignore(just(Token::Op(Op::Set)))
        .then(inline_expr.clone())
        .map(|(name, value)| Expr::Set { name, value })
}

fn if_parser<'tokens, 'src: 'tokens>(
    inline_expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
    block: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Spanned<Expr<'src>>) {
    just(Token::Keyword(Keyword::If))
        .ignore_then(inline_expr.clone())
        .then(block.clone())
        .then(
            just(Token::Keyword(Keyword::Else))
                .ignore_then(block)
                .or_not(),
        )
        .map_with_span(|((condition, body), r#else), span| {
            (
                Expr::If {
                    condition,
                    body: Box::new(body),
                    r#else: Box::new(r#else),
                },
                span,
            )
        })
}

fn fn_parser<'tokens, 'src: 'tokens>(
    ident_raw: parser!('tokens, 'src, &'src str),
    block: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Spanned<Expr<'src>>) {
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
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
        )
        .then(
            just(Token::Op(Op::Arrow))
                .ignore_then(ident_raw.clone())
                .or_not(),
        )
        .then(block)
        .map_with_span(|(((name, args), return_type), body), span| {
            (
                Expr::Fn {
                    name,
                    args,
                    return_type,
                    body: Box::new(body),
                },
                span,
            )
        })
}

pub fn expr_parser<'tokens, 'src: 'tokens>() -> parser!('tokens, 'src, Vec<Spanned<Expr<'src>>>) {
    recursive(|expr| {
        let inline_expr = inline_expr_parser();

        let ident_raw = select! { Token::Word(ident) => ident };

        let r#let = let_parser(ident_raw.clone(), inline_expr.clone());
        let r#return = return_parser(inline_expr.clone());
        let set = set_parser(ident_raw.clone(), inline_expr.clone());
        let inline = inline_expr.clone().map(|expr| Expr::Inline(expr));

        let line_expr = r#let.or(r#return).or(set).or(inline);

        let line_expr = line_expr
            .then_ignore(just(Token::Ctrl(';')))
            .map_with_span(|line, span| (line, span));

        let block = expr
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            .map_with_span(|block, span| (Expr::Block(block), span));

        let r#if = if_parser(inline_expr.clone(), block.clone());
        let r#fn = fn_parser(ident_raw.clone(), block.clone());

        r#fn.or(r#if)
            .or(block)
            .or(line_expr)
            .repeated()
            .collect::<Vec<_>>()
    })
}
