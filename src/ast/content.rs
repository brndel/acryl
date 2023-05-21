use crate::parser::Spanned;

use super::{Expr, Op};

#[derive(Debug)]
pub enum ContentToken<'src> {
    Word(Spanned<&'src str>),
    Expr(Spanned<Expr<'src>>),
    Op(Spanned<Op<'src>>),
    Block(Spanned<Vec<Spanned<Self>>>),
}
