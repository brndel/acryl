use super::{ty::Type};

#[derive(Debug, Clone)]
pub struct FunctionSignature<'src> {
    name: &'src str,
    args: Vec<Type<'src>>,
}

impl<'src> FunctionSignature<'src> {
    pub const fn new(name: &'src str, args: Vec<Type<'src>>) -> Self {
        Self { name, args }
    }
}
