pub mod ast;
mod code;
mod content;
pub mod file;
mod lexer;

use std::fmt::{Debug, Display};

use chumsky::{
    input::SpannedInput,
    prelude::{Input, Rich},
    primitive::{end, just},
    select,
    span::SimpleSpan,
    IterParser, Parser,
};
use code::code_token_parser;
use content::content_parser;
use file::DocFile;
use lexer::Token;

type Span = SimpleSpan<usize>;
type Spanned<T> = (T, Span);

#[macro_export]
macro_rules! parser {
    ($lifetime:lifetime, $input:ty, $output:ty) => {
    impl chumsky::Parser<
        $lifetime,
        crate::SpannedInput<$input, crate::Span, &$lifetime [crate::Spanned<$input>]>,
        $output,
        chumsky::extra::Err<chumsky::error::Rich<$lifetime, $input, crate::Span>>,
    > + Clone
    };
}

#[derive(Debug)]
pub enum ParsedFile<'src> {
    Doc(DocFile<'src>),
    Logic,
}

fn doc_parser<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, DocFile<'src>) {
    let field = select! {Token::Word(s) => s}
        .then_ignore(just(Token::Ctrl(':')))
        .then(code_token_parser());

    let header_fields = field
        .separated_by(just(Token::Ctrl(',')))
        .allow_trailing()
        .collect();

    let header = just(Token::Word("doc"))
        .ignore_then(header_fields.delimited_by(just(Token::Ctrl('{')), just(Token::Ctrl('}'))));

    let content = content_parser();

    header
        .then(content)
        .map(|(header, tokens)| DocFile::new(header, tokens))
}

fn parser<'src: 'tokens, 'tokens>() -> parser!('tokens, Token<'src>, ParsedFile<'src>) {
    doc_parser().map(ParsedFile::Doc).then_ignore(end())
}

pub fn parse<'src>(source: &'src str) -> Option<ParsedFile<'src>> {
    let (tokens, errors) = lexer::lexer().parse(source).into_output_errors();

    if !errors.is_empty() {
        println!("----- Lexer errors -----");
        for error in errors {
            print_error(source, &error);
        }
    }

    let tokens = tokens?;

    let (output, errors) = parser()
        .parse(tokens.spanned((source.len()..source.len()).into()))
        .into_output_errors();

    if !errors.is_empty() {
        println!("----- Parser errors -----");
        for error in errors {
            print_error(source, &error);
        }
    }

    output
}

fn print_error<'src, T: Debug>(source: &'src str, error: &Rich<'src, T>) {
    println!("{:?}", error);
    let slice = get_line_slice(source, error.span());
    println!("{}", slice);
}

struct LineSlice<'a> {
    before: &'a str,
    inner: &'a str,
    after: &'a str,
    line: usize,
    column: usize,
}

impl<'a> Display for LineSlice<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\x1b[1;30;41m{}\x1b[0m{} \x1b[1m{}:{}\x1b[0m",
            self.before, self.inner, self.after, self.line, self.column
        )?;
        writeln!(f, "{}\x1b[1;30;41m↑ Here\x1b[0m", " ".repeat(self.column))
    }
}

fn get_line_slice<'src>(source: &'src str, span: &SimpleSpan) -> LineSlice<'src> {
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
                None => {}
                _ => {}
            }

            position -= 1;
        }

        counter
    }

    let inner_start = span.start;
    let inner_end = span.end;

    let line_start_position = expand(span.start, -1, &chars);
    let line_end_position = expand(span.end - 1, 1, &chars) + 1;

    let column = span.start - line_start_position;
    let line = count(span.start, &chars) + 1;

    return LineSlice {
        before: &source[line_start_position..inner_start],
        inner: &source[inner_start..inner_end],
        after: &source[inner_end..line_end_position],
        line,
        column,
    };
}
