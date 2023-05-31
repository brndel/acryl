use std::rc::Rc;

use crate::{ast::Instr, parser::Spanned};

use super::{stack::StackStorage, ty::Type, value::Value, function::FunctionSignature};

pub struct Evaluator<'tokens, 'src: 'tokens> {
    instructions: &'tokens [Spanned<Instr<'src>>],
    storage: Vec<StorageEntry<'tokens, 'src>>,
}

struct StorageScope<'tokens, 'src: 'tokens> {
    before: Option<&'tokens mut Self>,
    entries: Vec<StorageEntry<'tokens, 'src>>
}

impl<'tokens, 'src: 'tokens> StorageScope<'tokens, 'src> {

    fn get_var<'a>(&'a mut self, name: &'src str) -> Option<&'a mut VariableEntry<'src>> {
        for entry in self.entries.iter_mut().rev() {
            if let StorageEntry::Variable(var) = entry {
                if var.name == name {
                    return Some(var)
                }
            }
        }

        if let Some(before) = &mut self.before {
            return before.get_var(name);
        }

        None
    }

}

enum StorageEntry<'tokens, 'src: 'tokens> {
    Variable(VariableEntry<'src>),
    Type(TypeEntry<'src>),
    Function(FunctionEntry<'src, 'tokens>),
}

struct VariableEntry<'src> {
    name: &'src str,
    ty: Type<'src>,
    value: Box<dyn Typed<'src>>,
}

struct TypeEntry<'src> {
    name: &'src str,
    ty: Type<'src>,
}

#[derive(Debug, Clone)]
pub struct FunctionEntry<'tokens, 'src: 'tokens> {
    signature: FunctionSignature<'src>,
    instr: &'tokens Instr<'src>
}

impl<'tokens, 'src: 'tokens> FunctionEntry<'tokens, 'src> {

    fn run(&self, evaluator: &mut Evaluator) -> Option<Value> {
        None
    }

}

trait Typed<'src> {}

impl Typed<'_> for i64 {}

struct TypeStruct<'src> {
    name: &'src str,
    values: Vec<(&'src str, Box<dyn Typed<'src>>)>,
}

impl<'src: 'tokens, 'tokens> Evaluator<'src, 'tokens> {
    pub fn new(instructions: &'tokens [Spanned<Instr<'src>>]) -> Self {
        Self { instructions, storage: Vec::new() }
    }

    pub fn eval(&mut self) {

        let func = self.get_function(&FunctionSignature::new("print", vec![Type::Int]));

        if let Some(func) = func {
            func.run(self);
        }

    }

    pub fn get_type(&self, name: &'src str) -> Option<&'tokens TypeEntry> {
        for entry in self.storage.iter().rev() {
            if let StorageEntry::Type(ty) = entry {
                if ty.name == name {
                    return Some(ty);
                }
            }
        }
        None
    }

    pub fn add_function(&mut self, signature: FunctionSignature<'src>, instr: &'tokens Instr<'src>) {
        let entry = StorageEntry::Function(FunctionEntry { signature, instr });
        self.storage.push(entry);
    }

    pub fn get_function(&self, signature: &FunctionSignature<'src>) -> Option<FunctionEntry<'tokens, 'src>> {
        for entry in self.storage.iter().rev() {
            if let StorageEntry::Function(func) = entry {
                if &func.signature == signature {
                    return Some(func.clone());
                }
            }
        }
        None
    }
}