use std::mem::discriminant;

#[derive(Debug, Clone)]
pub enum Type<'src> {
    Any,
    Null,
    Bool,
    Int,
    Float,
    String,
    Array {
        ty: Box<Self>,
    },
    Struct {
        name: &'src str,
        fields: Vec<(&'src str, Self)>,
    },
    Optional(Box<Self>),
}

impl<'src> Type<'src> {
    /**
       Wheter a variable of type `self` can hold a [`Value`] of type `other`
    */
    pub fn can_hold(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Any, _) => true,
            (Self::Array { ty: l_ty }, Self::Array { ty: r_ty }) => l_ty == r_ty,
            (
                Self::Struct {
                    name: l_name,
                    fields: l_fields,
                },
                Self::Struct {
                    name: r_name,
                    fields: r_fields,
                },
            ) => l_name == r_name && l_fields.iter().zip(r_fields.iter()).all(|(a, b)| a.can_hold(b)),
            (Self::Optional(l_ty), Self::Optional(r_ty)) => l_ty.can_hold(r_ty),
            (Self::Optional(l_ty), r_ty) => l_ty.can_hold(r_ty),
            _ => discriminant(self) == discriminant(other),
        }
    }
}
