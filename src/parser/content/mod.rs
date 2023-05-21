use chumsky::{primitive::just, recursive::recursive, select, IterParser, Parser};

use crate::{
    ast::{ContentToken, Expr},
    parser::lexer::Token,
};

use super::{shared::fn_call::fn_call_parser, Spanned};

pub fn content_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Vec<Spanned<ContentToken<'src>>>) {
    recursive(|token| {
        let word = select! {
            Token::Word(word) => word,
        }
        .map_with_span(|word, span| ContentToken::Word((word, span)));

        let fn_call = just(Token::Escape).ignore_then(fn_call_parser(expr.clone()).map_with_span(
            |fn_call, span| ContentToken::Expr((Expr::FnCall((fn_call, span)), span)),
        ));

        let expr = expr
            .clone()
            .delimited_by(
                just(Token::Escape).then(just(Token::Ctrl('('))),
                just(Token::Ctrl(')')),
            )
            .map_with_span(|(expr, _), span| ContentToken::Expr((expr, span)));

        let block = token
            .delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')))
            .map_with_span(|block, span| ContentToken::Block((block, span)));

        let op = select! {
            Token::Op(op) => op,
        }
        .map_with_span(|op, span| ContentToken::Op((op, span)));

        let literal = select! {
            Token::Num(slice) => slice,
            Token::NumHex(slice) => slice,
            Token::Str(slice) => slice,
        }
        .map_with_span(|slice, span| ContentToken::Word((slice, span)));

        let token = fn_call.or(expr).or(word).or(literal).or(op).or(block);

        let token = token.map_with_span(|token, span| (token, span));

        token.repeated().collect::<Vec<_>>()
    })
}
