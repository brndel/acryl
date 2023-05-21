use chumsky::{
    extra,
    input::SpannedInput,
    prelude::{Input, Rich},
    span::SimpleSpan,
    Parser, primitive::end,
};

#[macro_export]
macro_rules! parser {
    ($tokens: lifetime, $src: lifetime, $output: ty) => {
        impl chumsky::Parser<
            $tokens,
            crate::parser::ParserInput<$tokens, $src>,
            $output,
            crate::parser::AcrylError<$tokens, crate::parser::lexer::Token<$src>>
        > + Clone + $tokens
    };
}

mod code;
mod content;
pub mod lexer;
mod shared;

use crate::ast::Instr;

use self::lexer::{lexer, Token};

pub type Span = SimpleSpan<usize>;
pub type AcrylError<'a, T> = extra::Err<Rich<'a, T, Span>>;

pub type ParserInput<'tokens, 'src> =
    SpannedInput<Token<'src>, Span, &'tokens [(Token<'src>, Span)]>;

pub type Spanned<T> = (T, Span);

pub fn parse<'src>(source: &'src str) -> Vec<Spanned<Instr<'src>>> {
    let (tokens, errs) = lexer().parse(source).into_output_errors();

    for err in errs {
        println!("ERROR {}", err);
    }

    if let Some(tokens) = tokens {
        let (expr, errs) = code::code_parser().then_ignore(end())
            .parse(
                tokens
                    .as_slice()
                    .spanned((source.len()..source.len()).into()),
            )
            .into_output_errors();

        for err in errs {
            let (slice, (line, col)) = get_line_slice(source, err.span());
            println!("ERROR at line {}:{} ( {:?} )", line, col, err.span());
            println!("{:?}", err.reason());
            println!("{}", slice);
            println!("{}↑ Here", " ".repeat(col));
        }

        if let Some(expr) = expr {
            return expr;
        }
    }

    Vec::default()
}

fn get_line_slice<'src>(source: &'src str, span: &SimpleSpan) -> (&'src str, (usize, usize)) {
    let chars: Vec<_> = source.chars().collect();

    fn expand(start: usize, direction: isize, chars: &Vec<char>) -> usize {
        let mut position = start;

        loop {
            if position == 0 || position == (chars.len()) as usize {
                return position;
            }

            let next_pos = ((position as isize) + direction) as usize;
            if let Some(c) = chars.get(next_pos) {
                if c == &'\n' {
                    return position;
                }

                position = next_pos;
            } else {
                return position;
            }
        }
    }

    fn count(start: usize, chars: &Vec<char>) -> usize {
        let mut position = start;
        let mut counter = 0;

        while position > 1 {
            match chars.get(position) {
                Some('\n') => counter += 1,
                None => {},
                _ => {},
            }

            position -= 1;
        }

        counter
    }

    let start_position = expand(span.start, -1, &chars);
    let end_position = expand(span.end-1, 1, &chars)+1;

    let col = span.start - start_position;
    let line = count(span.start, &chars) + 1;

    return (&source[start_position..end_position], (line, col));
}
