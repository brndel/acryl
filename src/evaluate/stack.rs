use super::{function::FunctionSignature, ty::Type, value::Value, EvalResult};

pub struct StackStorage<'src> {
    entries: Vec<StackEntry<'src>>,
}

type EvalFunction<'src> = dyn Fn(Vec<Value<'src>>) -> Value<'src>;

pub enum StackEntry<'src> {
    Variable {
        name: &'src str,
        ty: Type<'src>,
        value: Value<'src>,
    },
    Function {
        signature: FunctionSignature<'src>,
        eval: Box<EvalFunction<'src>>,
    },
}

pub struct StackPointer {
    position: usize,
}

mod default_functions {
    use crate::evaluate::value::Value;

    pub fn print<'src>(input: Vec<Value<'src>>) -> Value<'src> {
        let value = input.get(0).expect("INTERNAL ERROR");
        println!("PRINT '{:?}'", value);
        Value::Null
    }

    pub fn add_int<'src>(input: Vec<Value<'src>>) -> Value<'src> {
        let a = input.get(0).expect("INTERNAL ERROR");
        let b = input.get(1).expect("INTERNAL ERROR");

        if let Value::Int(a) = a {
            if let Value::Int(b) = b {
                return Value::Int(a + b);
            }
        }

        panic!("INTERNAL ERROR");
    }

    pub fn sub_int<'src>(input: Vec<Value<'src>>) -> Value<'src> {
        let a = input.get(0).expect("INTERNAL ERROR");
        let b = input.get(1).expect("INTERNAL ERROR");

        if let Value::Int(a) = a {
            if let Value::Int(b) = b {
                return Value::Int(a - b);
            }
        }

        panic!("INTERNAL ERROR");
    }

    pub fn mul_int<'src>(input: Vec<Value<'src>>) -> Value<'src> {
        let a = input.get(0).expect("INTERNAL ERROR");
        let b = input.get(1).expect("INTERNAL ERROR");

        if let Value::Int(a) = a {
            if let Value::Int(b) = b {
                return Value::Int(a * b);
            }
        }

        panic!("INTERNAL ERROR");
    }

    pub fn div_int<'src>(input: Vec<Value<'src>>) -> Value<'src> {
        let a = input.get(0).expect("INTERNAL ERROR");
        let b = input.get(1).expect("INTERNAL ERROR");

        if let Value::Int(a) = a {
            if let Value::Int(b) = b {
                return Value::Int(a / b);
            }
        }

        panic!("INTERNAL ERROR");
    }
}

impl<'src> StackStorage<'src> {
    pub fn new() -> Self {
        let entries = vec![
            StackEntry::Function {
                signature: FunctionSignature::new("print", vec![Type::Int]),
                eval: Box::new(default_functions::print),
            },
            StackEntry::Function {
                signature: FunctionSignature::new("+", vec![Type::Int, Type::Int]),
                eval: Box::new(default_functions::add_int),
            },
            StackEntry::Function {
                signature: FunctionSignature::new("-", vec![Type::Int, Type::Int]),
                eval: Box::new(default_functions::sub_int),
            },
            StackEntry::Function {
                signature: FunctionSignature::new("*", vec![Type::Int, Type::Int]),
                eval: Box::new(default_functions::mul_int),
            },
            StackEntry::Function {
                signature: FunctionSignature::new("/", vec![Type::Int, Type::Int]),
                eval: Box::new(default_functions::div_int),
            },
        ];
        StackStorage { entries }
    }

    pub fn get_var(&self, name: &'src str) -> Option<Value<'src>> {
        for entry in self.entries.iter().rev() {
            if let StackEntry::Variable {
                name: var_name,
                ty,
                value,
            } = entry
            {
                if name == *var_name {
                    return Some(value.clone());
                }
            }
        }

        None
    }

    pub fn set_var(&mut self, name: &'src str, value: Value<'src>) -> bool {
        for entry in self.entries.iter_mut().rev() {
            if let StackEntry::Variable { name: var_name, ty, .. } = entry {
                if name == *var_name {
                    if ty.can_hold(&value.get_type()) {
                        *entry = StackEntry::Variable { name, ty: ty.to_owned(), value };
                    }
                    // let r = &value;
                    return true;
                }
            }
        }

        false
    }

    pub fn add_var(&mut self, name: &'src str, value: Value<'src>, ty: Option<Type<'src>>) {
        let ty = ty.unwrap_or_else(|| value.get_type());
        self.entries.push(StackEntry::Variable { name, value, ty })
    }

    pub fn get_function(&self, signature: &FunctionSignature) -> Option<&Box<EvalFunction<'src>>> {
        for entry in self.entries.iter().rev() {
            if let StackEntry::Function {
                signature: fn_signature,
                eval,
            } = entry
            {
                if signature == fn_signature {
                    return Some(eval);
                }
            }
        }

        None
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
