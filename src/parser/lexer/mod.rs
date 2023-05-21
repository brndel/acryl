mod keyword;
mod token;

use chumsky::{
    primitive::{any, end, just, none_of, one_of},
    text, IterParser, Parser,
};

use crate::ast::Op;

pub use self::keyword::Keyword;
pub use self::token::Token;

use super::{AcrylError, Spanned};

fn num<'src>() -> impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone {
    text::int(10)
        .then(just('.').then(text::int(10)).or_not())
        .map_slice(|s| Token::Num(s))
}

fn num_hex<'src>() -> impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone {
    just('#')
        .ignore_then(text::int(16))
        .map_slice(|s| Token::NumHex(s))
}

fn string<'src>() -> impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone {
    just('"')
        .ignore_then(none_of('"').repeated().slice())
        .then_ignore(just('"'))
        .map(|s: &str| Token::Str(s))
}

fn op<'src>() -> impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone {
    one_of(Op::ALLOWED)
        .repeated()
        .at_least(1)
        .map_slice(|s| Token::Op(Op(s)))
}

fn word<'src>(
    non_word: &(impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone),
) -> impl Parser<'src, &'src str, Token<'src>, AcrylError<'src, char>> + Clone {
    any()
        .and_is(non_word.to_owned().not())
        .filter(|c| !c.is_whitespace())
        .repeated()
        .at_least(1)
        .map_slice(|word| {
            if let Some(keyword) = Keyword::parse(word) {
                Token::Keyword(keyword)
            } else {
                Token::Word(word)
            }
        })
}

pub fn lexer<'src>(
) -> impl Parser<'src, &'src str, Vec<Spanned<Token<'src>>>, AcrylError<'src, char>> {
    let num = num();
    let num_hex = num_hex();
    let string = string();
    let op = op();
    let ctrl = one_of("()[]{}:;,").map(Token::Ctrl);
    let escape = just('\\').to(Token::Escape);

    let non_word = num.or(num_hex).or(string).or(op).or(ctrl).or(escape);

    let word = word(&non_word);

    let token = non_word.or(word);

    let comment = just("//").then(none_of('\n').repeated()).padded();

    token
        .map_with_span(|tok, span| (tok, span))
        .padded_by(comment.repeated())
        .padded()
        .repeated()
        .collect()
        .then_ignore(end())
}
