use crate::ast::Op;

use super::keyword::Keyword;

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
