use std::fmt::Display;

use chumsky::{primitive::just, recursive::recursive, select, Parser};

use crate::parser::{
    content::{content_parser, ContentToken},
    lexer::{Keyword, Op, Token},
    shared::fn_call::{self, fn_call_parser, FnCall},
    Span, Spanned,
};

#[derive(Debug)]
pub enum Literal<'src> {
    Bool(bool),
    NumF(f64),
    NumI(i64),
    Str(&'src str),
}

impl<'src> Literal<'src> {
    pub fn parse_num(num: &'src str) -> Self {
        if num.contains('.') {
            Self::NumF(num.parse().unwrap())
        } else {
            Self::NumI(num.parse().unwrap())
        }
    }
}

impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Bool(v) => write!(f, "{}", v),
            Literal::NumF(v) => write!(f, "{}", v),
            Literal::NumI(v) => write!(f, "{}", v),
            Literal::Str(v) => write!(f, "'{}'", v),
        }
    }
}

#[derive(Debug)]
pub enum InlineExpr<'src> {
    Error,
    Literal(Literal<'src>),
    Ident(&'src str),
    FnCall(FnCall<'src>),

    Binary(Box<Spanned<Self>>, Op<'src>, Box<Spanned<Self>>),
    Content(Vec<Spanned<ContentToken<'src>>>),
}

impl Display for InlineExpr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InlineExpr::Error => write!(f, "ERROR"),
            InlineExpr::Literal(literal) => write!(f, "{}", literal),
            InlineExpr::Ident(ident) => write!(f, "{}", ident),
            InlineExpr::FnCall(fn_call) => write!(f, "{}", fn_call),
            InlineExpr::Binary(a, op, b) => write!(f, "({} {} {})", a.0, op, b.0),
            InlineExpr::Content(content) => {
                write!(f, "\\{{")?;
                for token in content {
                    write!(f, "{:?}", token)?;
                }
                write!(f, "}}")
            }
        }
    }
}

fn fold_binary<'src>(
    a: Spanned<InlineExpr<'src>>,
    other: (Op<'src>, Spanned<InlineExpr<'src>>),
) -> Spanned<InlineExpr<'src>> {
    let (op, b) = other;
    let span = a.1.start..b.1.end;
    (
        InlineExpr::Binary(Box::new(a), op, Box::new(b)),
        span.into(),
    )
}

fn op_parser<'tokens, 'src: 'tokens>(
    op_array: Option<&'static [Op<'static>]>,
    atom: parser!('tokens, 'src, Spanned<InlineExpr<'src>>),
) -> parser!('tokens, 'src, Spanned<InlineExpr<'src>>) {
    let op = select! {
        Token::Op(op) if op_array.map(|op_array| op_array.contains(&op)).unwrap_or(true)  => op,
    };

    atom.clone().foldl(op.then(atom).repeated(), fold_binary)
}

pub fn inline_expr_parser<'tokens, 'src: 'tokens>(
) -> parser!('tokens, 'src, Spanned<InlineExpr<'src>>) {
    recursive(|expr| {
        let literal = select! {
            Token::Keyword(Keyword::True) => InlineExpr::Literal(Literal::Bool(true)),
            Token::Keyword(Keyword::False) => InlineExpr::Literal(Literal::Bool(false)),
            Token::Num(n) => InlineExpr::Literal(Literal::parse_num(n)),
            Token::Str(s) => InlineExpr::Literal(Literal::Str(s)),
        };

        let ident = select! { Token::Word(ident) => InlineExpr::Ident(ident) };

        let fn_call = fn_call_parser(expr.clone()).map(|fn_call| InlineExpr::FnCall(fn_call));

        let content = content_parser(expr.clone())
            .map(|content| InlineExpr::Content(content))
            .delimited_by(
                just(Token::Escape).then(just(Token::Ctrl('{'))),
                just(Token::Ctrl('}')),
            );

        let atom = fn_call.or(content).or(literal).or(ident);
        let atom = atom
            .map_with_span(|expr, span: Span| (expr, span))
            .or(expr.delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))));

        let atom = op_parser(Some(Op::high_priority()), atom);
        let atom = op_parser(Some(Op::mid_priority()), atom);
        let atom = op_parser(Some(Op::low_priority()), atom);
        let atom = op_parser(None, atom);

        atom
    })
}
