use chumsky::{
    extra::Err,
    prelude::Rich,
    primitive::{any, end, just, none_of, one_of},
    recovery::skip_then_retry_until,
    text::{self},
    Parser, IterParser,
};

mod token;
use crate::{Span, Spanned, ast::Op};

pub use self::token::Token;

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, Err<Rich<'src, char, Span>>> {
    let num = text::int(10)
        .then(just('.').then(text::digits(10)).or_not())
        .map_slice(Token::Num);

    let r#str = just('"')
        .ignore_then(none_of('"').repeated())
        .then_ignore(just('"'))
        .map_slice(|s: &str| Token::Str(&s[1..s.len()-1]));

    let word = any()
        .filter(char::is_ascii_alphabetic)
        .then(any().filter(char::is_ascii_alphanumeric).repeated())
        .map_slice(Token::Word);

    let ctrl = one_of("()[]{};:,").map(Token::Ctrl);

    let op = one_of(Op::ALLOWED).repeated().at_least(1).map_slice(Token::Op);

    let escape = just("\\").to(Token::Escape);

    let token = num.or(r#str).or(word).or(ctrl).or(op).or(escape);

    token
        .map_with_span(|token, span| (token, span))
        .padded()
        .recover_with(skip_then_retry_until(any().ignored(), end()))
        .repeated()
        .collect()
}
