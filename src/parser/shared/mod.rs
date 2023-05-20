use chumsky::{select, Parser};

use crate::parser::lexer::Token;

use super::Spanned;

pub mod fn_call;


pub fn raw_ident_parser<'tokens, 'src: 'tokens>() -> parser!('tokens, 'src, Spanned<&'src str>) {
    select! { Token::Word(word) => word }.map_with_span(|ident, span| (ident, span))
}