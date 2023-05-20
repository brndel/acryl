use std::fmt::Display;

use chumsky::{primitive::just, IterParser, Parser};

use crate::parser::{code::InlineExpr, lexer::Token, Spanned};

use super::raw_ident_parser;

#[derive(Debug)]
pub struct FnCall<'src> {
    name: Spanned<&'src str>,
    args: Vec<Spanned<FnCallArg<'src>>>,
}

impl<'src> Display for FnCall<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name.0)?;

        for arg in &self.args {
            write!(f, "{}, ", arg.0)?;
        }

        write!(f, ")")
    }
}

#[derive(Debug)]
pub struct FnCallArg<'src> {
    name: Option<Spanned<&'src str>>,
    value: Spanned<InlineExpr<'src>>,
}

impl<'src> Display for FnCallArg<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = self.name {
            write!(f, "{}", name.0)?;
        }

        write!(f, "{}", self.value.0)
    }
}

fn call_arg_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, Spanned<FnCallArg<'src>>) {
    raw_ident_parser()
        .then_ignore(just(Token::Ctrl(':')))
        .or_not()
        .then(expr)
        .map_with_span(|(name, value), span| (FnCallArg { name, value }, span))
}

pub fn fn_call_parser<'tokens, 'src: 'tokens>(
    expr: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, FnCall<'src>) {
    raw_ident_parser()
        .then(
            call_arg_parser(expr)
                .separated_by(just(Token::Ctrl(',')))
                .allow_trailing()
                .collect::<Vec<_>>()
                .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))),
        )
        .map(|(name, args)| FnCall { name, args })
}
