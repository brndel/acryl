use std::fmt::Display;

use crate::parser::Spanned;

use super::{content::ContentToken, literal::Literal, op::Op};

/**
An inline Expression like an identifier, literal or function call
*/
#[derive(Debug)]
pub enum Expr<'src> {
    Literal(Spanned<Literal<'src>>),
    Ident(Spanned<&'src str>),
    FnCall(Spanned<FnCall<'src>>),

    Unary(Spanned<Op<'src>>, Box<Spanned<Self>>),
    Binary(Box<Spanned<Self>>, Spanned<Op<'src>>, Box<Spanned<Self>>),

    Content(Spanned<Vec<Spanned<ContentToken<'src>>>>),
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal((literal, _)) => write!(f, "{}", literal),
            Expr::Ident((ident, _)) => write!(f, "{}", ident),
            Expr::FnCall((fn_call, _)) => write!(f, "{}", fn_call),
            Expr::Unary(op, a) => write!(f, "({} {})", op.0, a.0),
            Expr::Binary(a, op, b) => write!(f, "({} {} {})", a.0, op.0, b.0),
            Expr::Content((content, _)) => {
                write!(f, "\\{{")?;
                for token in content {
                    write!(f, "{:?}", token)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[derive(Debug)]
pub struct FnCall<'src> {
    name: Spanned<&'src str>,
    args: Spanned<Vec<Spanned<Expr<'src>>>>,
}

impl<'src> Display for FnCall<'src> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}(", self.name.0)?;

        for arg in &self.args.0 {
            write!(f, "{}, ", arg.0)?;
        }

        write!(f, ")")
    }
}

impl<'src> FnCall<'src> {
    pub fn new(name: Spanned<&'src str>, args: Spanned<Vec<Spanned<Expr<'src>>>>) -> Self {
        Self { name, args }
    }
}
