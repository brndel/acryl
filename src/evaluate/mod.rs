use crate::parser::{Spanned, Span};

use self::{stack::StackStorage, value::Value};

pub mod stack;
pub mod value;
pub mod function;
pub mod ty;
pub mod evaluator;

pub type EvalResult<T> = Result<T, Spanned<String>>;

pub trait Eval<'src> {
    fn eval(&'src self, storage: &mut StackStorage<'src>) -> EvalResult<Value>;
}

impl<'src, E> Eval<'src> for (E, Span) where E: Eval<'src> {
    fn eval(&'src self, storage: &mut StackStorage<'src>) -> EvalResult<Value> {
        self.0.eval(storage)
    }
}

impl<'src, E> Eval<'src> for Box<E> where E: Eval<'src> {
    fn eval(&'src self, storage: &mut StackStorage<'src>) -> EvalResult<Value> {
        (**self).eval(storage)
    }
}