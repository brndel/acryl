use std::fmt::Display;

use crate::{
    evaluate::{function::FunctionSignature, stack::StackStorage, value::Value, Eval, EvalResult},
    parser::{Span, Spanned},
};

use super::{content::ContentToken, fn_call::FnCall, literal::Literal, op::Op};

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

fn call_function<'src, E: Eval<'src>>(
    storage: &mut StackStorage<'src>,
    name: &'src str,
    args: Vec<&'src E>,
    span: &Span,
) -> EvalResult<Value<'src>> {
    let mut parameters: Vec<Value> = Vec::new();

    for expr in args {
        parameters.push(expr.eval(storage)?);
    }

    let signature = FunctionSignature::new(name, parameters.iter().map(|v| v.get_type()).collect());

    let eval = storage.get_function(&signature).ok_or_else(|| {
        (
            format!("could not find function of signature {:?}", signature),
            *span,
        )
    })?;

    let value = eval(parameters);

    Ok(value)
}

impl<'src> Eval<'src> for Expr<'src> {
    fn eval(&'src self, storage: &mut StackStorage<'src>) -> EvalResult<Value<'src>> {
        match self {
            Expr::Literal((literal, _)) => literal.eval(storage),
            Expr::Ident((name, span)) => storage
                .get_var(name)
                .map(|value| value.to_owned())
                .ok_or_else(|| (format!("variable '{}' not found", name), *span)),
            Expr::FnCall((fn_call, span)) => {
                call_function(storage, fn_call.get_name().0, fn_call.get_args(), span)
            }
            Expr::Unary(_, _) => todo!(),
            Expr::Binary(a, (op, span), b) => {
                let name = op.0;
                let args = vec![a, b];
                call_function(storage, name, args, span)
            },
            Expr::Content(_) => todo!(),
        }
    }
}
