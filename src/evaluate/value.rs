use super::ty::Type;

#[derive(Debug, Clone)]
pub enum Value<'src> {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Array {
        ty: Type<'src>,
        values: Vec<Self>,
    },
    Struct {
        name: &'src str,
        fields: Vec<(&'src str, Self)>,
    },
}

impl<'src> Value<'src> {
    pub fn get_type(&self) -> Type<'src> {
        match self {
            Value::Null => Type::Null,
            Value::Bool(_) => Type::Bool,
            Value::Int(_) => Type::Int,
            Value::Float(_) => Type::Float,
            Value::String(_) => Type::String,
            Value::Array { ty, .. } => Type::Array {
                ty: Box::new(ty.clone()),
            },
            Value::Struct { name, fields } => Type::Struct {
                name,
                fields: fields
                    .clone()
                    .into_iter()
                    .map(|(name, value)| (name, value.get_type()))
                    .collect(),
            },
        }
    }
}
