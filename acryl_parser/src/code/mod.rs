use chumsky::{primitive::any, Parser, prelude::Rich};

use crate::{ast::CodeToken, lexer::Token, parser};

// parser!('tokens, Token<'src>, CodeToken<'src>)
pub fn code_token_parser<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, CodeToken<'src>) {
    any().try_map(|token, span| {
        match token {
            Token::Word(word) => Ok(CodeToken::Ident(word)),
            Token::Num(num) => {
                if num.contains('.') {
                    match num.parse() {
                        Ok(num) => Ok(CodeToken::Float(num)),
                        Err(error) => Err(Rich::custom(span, format!("'{}' {}", num, error)))
                    }
                } else {
                    match num.parse() {
                        Ok(num) => Ok(CodeToken::Int(num)),
                        Err(error) => Err(Rich::custom(span, format!("'{}' {}", num, error)))
                    }
                }
            },
            Token::Str(content) => Ok(CodeToken::Str(content)),
            Token::Op(_) => todo!(),
            Token::Ctrl(_) => todo!(),
            Token::Escape => todo!(),
        }
    })
}