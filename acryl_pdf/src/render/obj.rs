use std::{borrow::Cow, io::Write};

use acryl_core::{unit::Pt, Vector2, VectorComponent, Area};

use super::{Context, PdfObjRef};

pub enum PdfObj {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    StringLiteral(Cow<'static, str>),
    HexString(Vec<u8>),
    Name(Cow<'static, str>),
    Array(Vec<Self>),
    Dict(Vec<(Cow<'static, str>, Self)>),
    Stream(Vec<u8>),
    Refernce(u64, u64),
}

#[macro_export]
macro_rules! pdf_dict {
    ($($k: expr => $v: expr),* $(,)?) => {
        PdfObj::Dict(vec![$( ($k.into(), $v), )*] )
    };
}

impl PdfObj {
    pub fn render<F: Write>(self, f: &mut F) -> std::io::Result<()> {
        match self {
            PdfObj::Null => write!(f, "null"),
            PdfObj::Bool(value) => write!(f, "{}", if value { "true" } else { "false" }),
            PdfObj::Int(value) => write!(f, "{}", value),
            PdfObj::UInt(value) => write!(f, "{}", value),
            PdfObj::Float(value) => write!(f, "{}", value),
            PdfObj::StringLiteral(value) => {
                write!(f, "({})", value.replace('(', "\\(").replace(')', "\\)"))
            }
            PdfObj::HexString(value) => {
                write!(f, "<")?;

                for v in value {
                    write!(f, "{:02x}", v)?;
                }

                write!(f, ">")
            }
            PdfObj::Name(value) => write!(f, "/{}", value),
            PdfObj::Array(values) => {
                write!(f, "[")?;

                for value in values {
                    value.render(f)?;
                    write!(f, " ")?;
                }

                write!(f, "]")
            }
            PdfObj::Dict(fields) => {
                writeln!(f, "<<")?;

                for (name, value) in fields {
                    write!(f, "/{} ", name)?;
                    value.render(f)?;
                    writeln!(f, "")?;
                }

                write!(f, ">>")
            }
            PdfObj::Stream(content) => {
                // let stream_content = stream.render()?;
                let dict = pdf_dict!("Length" => content.len().into());
                dict.render(f)?;
                writeln!(f, "")?;
                writeln!(f, "stream")?;
                f.write(&content)?;
                write!(f, "endstream")
            }
            PdfObj::Refernce(id, generation) => write!(f, "{} {} R", id, generation),
        }
    }

    pub fn add_to(self, context: &mut Context) -> PdfObjRef {
        context.add(self)
    }
}

macro_rules! impl_from_with_cast {
    ($($value:ident $cast_type:ty [ $($from_type:ty)* ] )*) => {
        $(
            $(
                impl From<$from_type> for PdfObj {
                    fn from(value: $from_type) -> Self {
                        Self::$value(value as $cast_type)
                    }
                }
            )*
        )*
    };
}

impl_from_with_cast!(
    Int i64 [isize i8 i16 i32 i64]
    UInt u64 [usize u8 u16 u32 u64]
    Float f64 [f32 f64]
);

impl<T: Into<PdfObj>> From<Vec<T>> for PdfObj {
    fn from(value: Vec<T>) -> Self {
        Self::Array(value.into_iter().map(|e| e.into()).collect())
    }
}

impl<T: Into<PdfObj>> From<Option<T>> for PdfObj {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(obj) => obj.into(),
            None => PdfObj::Null,
        }
    }
}

impl From<Pt> for PdfObj {
    fn from(value: Pt) -> Self {
        Self::Float(value.0)
    }
}

impl<T: Into<PdfObj> + VectorComponent> From<Vector2<T>> for PdfObj {
    fn from(value: Vector2<T>) -> Self {
        PdfObj::Array(vec![
            value.x.into(),
            value.y.into(),
        ])
    }
}

impl<T: Into<PdfObj> + VectorComponent> From<Area<T>> for PdfObj {
    fn from(value: Area<T>) -> Self {
        vec![
            value.position.x,
            value.position.y,
            value.position.x + value.size.x,
            value.position.x + value.size.y,
        ].into()
    }
}