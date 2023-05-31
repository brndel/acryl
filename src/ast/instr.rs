use std::fmt::Display;

use crate::{
    evaluate::{stack::StackStorage, value::Value, Eval, EvalResult},
    parser::Spanned,
};

use super::Expr;

/**
    An instruction
*/
#[derive(Debug)]
pub enum Instr<'src> {
    Expr(Spanned<Expr<'src>>),
    Let {
        name: Spanned<&'src str>,
        ty: Option<Spanned<&'src str>>,
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
    Struct {
        name: Spanned<&'src str>,
        fields: Vec<(&'src str, &'src str)>,
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
                ty,
                value: (value, _),
            } => write!(
                f,
                "let {} {}= {}",
                name,
                if let Some((ty, _)) = ty {
                    format!(": {} ", ty)
                } else {
                    String::new()
                },
                value
            ),
            Instr::Set {
                name: (name, _),
                value: (value, _),
            } => write!(f, "{} = {}", name, value),
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

impl<'src> Eval<'src> for Instr<'src> {
    fn eval(&'src self, storage: &mut StackStorage<'src>) -> EvalResult<Value<'src>> {
        match self {
            Instr::Expr((expr, _)) => expr.eval(storage),
            Instr::Let {
                name: (name, _),
                ty,
                value: (expr, _),
            } => {
                let value = expr.eval(storage)?;
                storage.add_var(name, value, None); // TODO: implement ty
                Ok(Value::Null)
            }
            Instr::Set {
                name: (name, span),
                value: (expr, _),
            } => {
                let value = expr.eval(storage)?;
                if storage.set_var(name, value) {
                    Ok(Value::Null)
                } else {
                    Err((format!("variable '{}' not defined", name), *span))
                }
            }
            Instr::If {
                condition,
                body,
                r#else,
            } => todo!(),
            Instr::Fn {
                name,
                args,
                return_type,
                body,
            } => todo!(),
            Instr::Return(_) => todo!(),
            Instr::Block(_) => todo!(),
        }
    }
}
