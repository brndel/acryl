use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::parser::lexer::Token;

use super::{
    code::InlineExpr,
    lexer::Op,
    shared::fn_call::{fn_call_parser, FnCall},
    Spanned,
};

#[derive(Debug)]
pub enum ContentToken<'src> {
    Word(&'src str),
    Op(Op<'src>),
    Block(Vec<Spanned<Self>>),
    Expr(InlineExpr<'src>),
}

pub fn content_parser<'tokens, 'src: 'tokens>(
    expr: impl chumsky::Parser<'tokens,crate::parser::ParserInput<'tokens,'src>,Spanned<InlineExpr<'src>> ,crate::parser::AcrylError<'tokens,crate::parser::lexer::Token<'src>> > +Clone + 'tokens,
) -> parser!('tokens, 'src, Vec<Spanned<ContentToken<'src>>>) {
    
    
    recursive(|token| {
        let word = select! {
            Token::Word(word) => ContentToken::Word(word)
        };

        let fn_call = just(Token::Escape).ignore_then(
            fn_call_parser(expr.clone()).map(|fn_call| ContentToken::Expr(InlineExpr::FnCall(fn_call)))
        );

        let inline_expr = expr.clone()
            .delimited_by(
                just(Token::Escape).then(just(Token::Ctrl('('))),
                just(Token::Ctrl(')')),
            )
            .map(|(expr, _)| ContentToken::Expr(expr));

        let block = token
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            .map(|block| ContentToken::Block(block));

        let op = select! {
            Token::Op(op) => ContentToken::Op(op)
        };
        let literal = select! {
            Token::Num(slice) => ContentToken::Word(slice),
            Token::NumHex(slice) => ContentToken::Word(slice),
            Token::Str(slice) => ContentToken::Word(slice),
        };

        let token = fn_call.or(word).or(literal).or(op).or(inline_expr).or(block);

        let token = token.map_with_span(|token, span| (token, span));

        token.repeated().collect::<Vec<_>>()
    })
}
