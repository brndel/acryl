use chumsky::{primitive::just, recursive::recursive, select, Parser};

use crate::{
    ast::{Expr, Literal, Op},
    parser::{
        content::content_parser,
        lexer::{Keyword, Token},
        shared::fn_call::fn_call_parser,
        Span, Spanned,
    },
};

fn fold_binary<'src>(
    a: Spanned<Expr<'src>>,
    other: (Spanned<Op<'src>>, Spanned<Expr<'src>>),
) -> Spanned<Expr<'src>> {
    let (op, b) = other;
    let span = a.1.start..b.1.end;
    (Expr::Binary(Box::new(a), op, Box::new(b)), span.into())
}

fn op_parser<'tokens, 'src: 'tokens>(
    op_array: Option<&'static [Op<'static>]>,
    atom: parser!('tokens, 'src, Spanned<Expr<'src>>),
) -> parser!('tokens, 'src, Spanned<Expr<'src>>) {
    let op = select! {
        Token::Op(op) if op_array.map(|op_array| op_array.contains(&&op)).unwrap_or(true)  => op,
    }
    .map_with_span(|op, span| (op, span));

    atom.clone().foldl(op.then(atom).repeated(), fold_binary)
}

pub fn expr_parser<'tokens, 'src: 'tokens>() -> parser!('tokens, 'src, Spanned<Expr<'src>>) {
    recursive(|expr| {
        let literal = select! {
            Token::Keyword(Keyword::Null) => Literal::Null,
            Token::Keyword(Keyword::True) => Literal::Bool(true),
            Token::Keyword(Keyword::False) => Literal::Bool(false),
            Token::Num(n) => Literal::parse_num(n),
            Token::Str(s) => Literal::Str(s),
        }
        .map_with_span(|literal, span| Expr::Literal((literal, span)));

        let ident = select! { Token::Word(ident) => ident }
            .map_with_span(|ident, span| Expr::Ident((ident, span)));

        let fn_call = fn_call_parser(expr.clone()).map_with_span(|fn_call, span| Expr::FnCall((fn_call, span)));

        let content = content_parser(expr.clone())
            .map_with_span(|content, span| Expr::Content((content, span)))
            .delimited_by(
                just(Token::Escape).then(just(Token::Ctrl('{'))),
                just(Token::Ctrl('}')),
            );

        let atom = fn_call.or(content).or(literal).or(ident);
        let atom = atom
            .map_with_span(|expr, span: Span| (expr, span))
            .or(expr.delimited_by(just(Token::Ctrl('(')), just(Token::Ctrl(')'))));

        let unary = select! {
            Token::Op(op) => op,
        }
        .map_with_span(|op, span| (op, span))
        .then(atom.clone())
        .map(|(op, a)| Expr::Unary(op, Box::new(a)))
        .map_with_span(|unary, span| (unary, span))
        .or(atom);

        let binary = op_parser(Some(Op::PRIORITY_HIGH), unary);
        let binary = op_parser(Some(Op::PRIORITY_MID), binary);
        let binary = op_parser(Some(Op::PRIORITY_LOW), binary);
        let binary = op_parser(None, binary);

        binary
    })
}
