use super::{function::FunctionInput, value::Value};

pub struct StackStorage<'src> {
    entries: Vec<StackEntry<'src>>,
}

pub enum StackEntry<'src> {
    Variable {
        name: &'src str,
        value: Value<'src>,
    },
    Function {
        name: &'src str,
        eval: Box<dyn Fn(&mut StackStorage, FunctionInput<'src>)>,
    },
}

pub struct StackPointer {
    position: usize,
}

mod default_functions {
    use crate::evaluate::{
        function::{FunctionInput, FunctionSignature},
        value::Type,
    };

    use super::StackStorage;

    pub fn print(stack: &mut StackStorage, input: FunctionInput) {
        let signature: FunctionSignature<'static> = FunctionSignature::new("print", vec![Type::Int]);
        
        input.matches_signature(&signature);
        let value = input.get_arg(0).expect("INTERNAL ERROR");
        println!("{:?}", value)
    }
}

impl<'src> StackStorage<'src> {
    pub fn new() -> Self {
        let entries = vec![StackEntry::Function {
            name: "print",
            eval: Box::new(default_functions::print),
        }];
        StackStorage { entries }
    }

    pub fn get_var(&self, name: &'src str) -> Option<&Value<'src>> {
        for entry in self.entries.iter().rev() {
            if let StackEntry::Variable {
                name: var_name,
                value,
            } = entry
            {
                if name == *var_name {
                    return Some(value);
                }
            }
        }

        None
    }

    pub fn set_var(&mut self, name: &'src str, value: Value<'src>) -> bool {
        for entry in self.entries.iter_mut().rev() {
            if let StackEntry::Variable { name: var_name, .. } = entry {
                if name == *var_name {
                    // let r = &value;
                    *entry = StackEntry::Variable { name, value };
                    return true;
                }
            }
        }

        false
    }

    pub fn add_var(&mut self, name: &'src str, value: Value<'src>) {
        self.entries.push(StackEntry::Variable { name, value })
    }

    pub fn get_pointer(&self) -> StackPointer {
        StackPointer {
            position: self.entries.len(),
        }
    }

    pub fn pop_pointer(&mut self, pointer: StackPointer) {
        self.entries.truncate(pointer.position);
    }
}
