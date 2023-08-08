use std::os::linux::raw;

use chumsky::{Parser, recursive::recursive, select, primitive::just, IterParser};

use crate::{lexer::Token, ast::{ContentToken, CodeToken, Argument}, parser, code::code_token_parser};

fn raw_ident<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, &'src str) {
    select! {
        Token::Word(s) => s,
    }
}

fn raw_token<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, &'src str) {
    select! {
        Token::Word(s) => s,
        Token::Num(s) => s,
        Token::Op(s) => s,
        Token::Str(s) => s,
    }
}

pub fn content_parser<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, Vec<ContentToken<'src>>) {
    recursive(|token| {
        let word = raw_token().map(ContentToken::Word);

        let raw_ident = raw_ident();

        let key = just(Token::Ctrl('[')).ignore_then(raw_token()).then_ignore(just(Token::Ctrl(']')));

        let argument = raw_ident
            .clone()
            .then_ignore(just(Token::Ctrl(':')))
            .or_not()
            .then(code_token_parser())
            .map(|(name, value)| {
                if let Some(name) = name {
                    Argument::Named { name, value }
                } else {
                    Argument::Unnamed(value)
                }
            });

        let fn_arguments = argument
            .separated_by(just(Token::Ctrl(',')))
            .collect()
            .delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')')));

        let fn_content = token.delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}')));

        let r#fn =
        just(Token::Escape)
            .ignore_then(raw_ident)
            .then(key.or_not())
            // raw_ident
            .then(fn_arguments.or_not())
            .then(fn_content.or_not())
            .map(|(((name, key), arguments), content)| ContentToken::Fn {
                name,
                key,
                arguments: arguments.unwrap_or_default(),
                content: content.unwrap_or_default(),
            });
            

        word.or(r#fn).repeated().collect()
    })
}