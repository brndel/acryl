use chumsky::{primitive::just, IterParser, Parser};

use crate::{
    ast::{Expr, FnCall},
    parser::{lexer::Token, Spanned},
};

use super::raw_ident_parser;

pub fn fn_call_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, FnCall<'src>) {
    raw_ident_parser()
        .then(
            expr.separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')))
                .map_with_span(|args, span| (args, span)),
        )
        .map(|(name, args)| FnCall::new(name, args))
}
