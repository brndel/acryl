use std::fmt::Display;

use crate::{parser::Spanned, evaluate::{stack::StackStorage, value::Value}};

use super::Expr;

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

    pub fn get_name(&self) -> &Spanned<&'src str> {
        &self.name
    }

    pub fn get_args(&self) -> Vec<&Spanned<Expr<'src>>> {
        self.args.0.iter().map(|v| v).collect()
    }
}
