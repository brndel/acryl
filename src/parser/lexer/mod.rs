use chumsky::{
    primitive::{any, end, just, none_of, one_of},
    text, IterParser, Parser,
};

use super::{AcrylError, Spanned};

#[derive(Debug, Clone, PartialEq)]
pub enum Token<'src> {
    Num(&'src str),
    NumHex(&'src str),
    Str(&'src str),
    Op(Op<'src>),
    Ctrl(char),
    Escape,
    Word(&'src str),
    Keyword(Keyword),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Op<'src> {
    Add,
    Subtract,
    Multiply,
    Divide,
    Set,
    Equals,
    NotEquals,
    Greater,
    GreaterEquals,
    Less,
    LessEquals,
    Not,
    Arrow,
    Custom(&'src str),
}

impl<'src> Op<'src> {
    pub const fn high_priority() -> &'static [Op<'static>] {
        return &[Op::Multiply, Op::Divide];
    }

    pub const fn mid_priority() -> &'static [Op<'static>] {
        return &[Op::Add, Op::Subtract];
    }

    pub const fn low_priority() -> &'static [Op<'static>] {
        return &[
            Op::Equals,
            Op::NotEquals,
            Op::Greater,
            Op::GreaterEquals,
            Op::Less,
            Op::LessEquals,
        ];
    }
}

impl<'src> std::fmt::Display for Op<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v = match self {
            Op::Add => "+",
            Op::Subtract => "-",
            Op::Multiply => "*",
            Op::Divide => "/",
            Op::Set => "=",
            Op::Equals => "==",
            Op::NotEquals => "!=",
            Op::Greater => ">",
            Op::GreaterEquals => ">=",
            Op::Less => "<",
            Op::LessEquals => "<=",
            Op::Not => "!",
            Op::Arrow => "->",
            Op::Custom(v) => v,
        };

        write!(f, "{}", v)
    }
}

impl<'src> From<&'src str> for Op<'src> {
    fn from(value: &'src str) -> Self {
        use Op::*;
        match value {
            "+" => Add,
            "-" => Subtract,
            "*" => Multiply,
            "/" => Divide,
            "=" => Set,
            "==" => Equals,
            "!=" => NotEquals,
            ">" => Greater,
            ">=" => GreaterEquals,
            "<" => Less,
            "<=" => LessEquals,
            "!" => Not,
            "->" => Arrow,
            _ => Custom(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    True,
    False,
    If,
    Else,
    Fn,
    Let,
    In,
    After,
    Return,
}

impl TryFrom<&str> for Keyword {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Keyword::*;
        match value {
            "true" => Ok(True),
            "false" => Ok(False),
            "if" => Ok(If),
            "else" => Ok(Else),
            "fn" => Ok(Fn),
            "let" => Ok(Let),
            "in" => Ok(In),
            "after" => Ok(After),
            "return" => Ok(Return),
            _ => Err(()),
        }
    }
}

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
    one_of("+-*/!=~^_<>?")
        .repeated()
        .at_least(1)
        .map_slice(|s| Token::Op(Op::from(s)))
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
            if let Ok(kwd) = Keyword::try_from(word) {
                Token::Keyword(kwd)
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
