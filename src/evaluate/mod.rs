use crate::parser::Spanned;

use self::{stack::StackStorage, value::Value};

pub mod stack;
pub mod value;
mod function;

pub type EvalResult<T> = Result<T, Spanned<String>>;

pub trait Eval<'src, I = (), O = Value<'src>> {
    fn eval(&self, storage: StackStorage, input: I) -> EvalResult<O>;
}