use std::fmt::Display;

use crate::parser::Spanned;

use super::Expr;

/**
    An instruction
*/
#[derive(Debug)]
pub enum Instr<'src> {
    Expr(Spanned<Expr<'src>>),
    Let {
        name: Spanned<&'src str>,
        value: Spanned<Expr<'src>>,
    },
    Set {
        name: Spanned<&'src str>,
        value: Spanned<Expr<'src>>,
    },
    If {
        condition: Spanned<Expr<'src>>,
        body: Box<Spanned<Self>>,
        r#else: Box<Option<Spanned<Self>>>,
    },
    Fn {
        name: Spanned<&'src str>,
        args: Spanned<Vec<(Spanned<&'src str>, Spanned<&'src str>)>>,
        return_type: Option<Spanned<&'src str>>,
        body: Box<Spanned<Self>>,
    },

    Return(Spanned<Expr<'src>>),

    Block(Vec<Spanned<Self>>),
}

impl Display for Instr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Instr::Expr((expr, _)) => write!(f, "{}", expr),
            Instr::Let {
                name: (name, _),
                value: (value, _),
            } => write!(f, "let {} = {}", name, value),
            Instr::Set { name: (name, _), value: (value, _) } => write!(f, "{} = {}", name, value),
            Instr::If {
                condition: (condition, _),
                body,
                r#else,
            } => {
                writeln!(f, "if {} {}", condition, body.0)?;
                if let Some(r#else) = r#else.as_ref() {
                    writeln!(f, "else {}", r#else.0)
                } else {
                    Ok(())
                }
            }
            Instr::Return(expr) => write!(f, "return {}", expr.0),
            Instr::Fn {
                name: (name, _),
                args: (args, _),
                return_type,
                body,
            } => {
                write!(f, "fn {}(", name)?;

                for ((name, _), (r#type, _)) in args {
                    write!(f, "{}: {}", name, r#type)?;
                }
                write!(f, ")")?;

                if let Some(return_type) = return_type {
                    write!(f, " -> {} ", return_type.0)?;
                }

                write!(f, "{}", body.0)
            }
            Instr::Block(expressions) => {
                writeln!(f, "{{")?;
                for expr in expressions {
                    writeln!(f, "{}", expr.0)?;
                }
                write!(f, "}} ")
            }
        }
    }
}
