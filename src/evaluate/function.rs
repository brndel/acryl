use super::value::{Type, Value};

pub struct FunctionInput<'src> {
    args: Vec<Value<'src>>,
}

impl<'src> FunctionInput<'src> {
    pub fn matches_signature(&self, signature: &FunctionSignature<'src>) -> bool {
        if self.args.len() != signature.args.len() {
            return false;
        }

        let i = 0;

        while i < self.args.len() {
            let ty = self.args[i].get_type();
            let expected = &signature.args[i];

            if ty != *expected {
                return false;
            }
        }

        true
    }

    pub fn get_arg(&self, index: usize) -> Option<&Value<'src>> {
        self.args.get(index)
    }
}

pub struct FunctionSignature<'src> {
    name: &'src str,
    args: Vec<Type<'src>>,
}

impl<'src> FunctionSignature<'src> {
    pub const fn new(name: &'src str, args: Vec<Type<'src>>) -> Self {
        Self { name, args }
    }
}
