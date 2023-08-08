use super::CodeToken;


pub enum Expr<'src> {
    Token(CodeToken<'src>),
    Unary(&'src str, Box<Self>),
    Binary(Box<Self>, &'src str, Box<Self>),
}