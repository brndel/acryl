use std::fmt::Write;

use crate::stream::{Stream, TextStream};

pub enum PdfObj {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    Float(f64),
    StringLiteral(String),
    Name(String),
    Array(Vec<Self>),
    Dict(Vec<(&'static str, Self)>),
    Stream(TextStream),
    Refernce(u64, u64),
}

impl PdfObj {
    pub fn render(self, f: &mut dyn Write) -> std::fmt::Result {
        match self {
            PdfObj::Null => write!(f, "null"),
            PdfObj::Bool(value) => write!(f, "{}", if value { "true" } else { "false" }),
            PdfObj::Int(value) => write!(f, "{}", value),
            PdfObj::UInt(value) => write!(f, "{}", value),
            PdfObj::Float(value) => write!(f, "{}", value),
            PdfObj::StringLiteral(value) => {
                write!(f, "({})", value.replace('(', "\\(").replace(')', "\\)"))
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
            PdfObj::Stream(stream) => {
                let stream_content = stream.render()?;
                let dict = PdfObj::Dict(vec![("Length", stream_content.len().into())]);
                dict.render(f)?;
                writeln!(f, "")?;
                writeln!(f, "{}", stream_content)?;

                writeln!(f, "endstream")
            }
            PdfObj::Refernce(id, generation) => write!(f, "{} {} R", id, generation),
        }
    }
}

impl Into<PdfObj> for isize {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for i8 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for i16 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for i32 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for i64 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for usize {
    fn into(self) -> PdfObj {
        PdfObj::UInt(self as u64)
    }
}

impl Into<PdfObj> for u8 {
    fn into(self) -> PdfObj {
        PdfObj::UInt(self as u64)
    }
}

impl Into<PdfObj> for u16 {
    fn into(self) -> PdfObj {
        PdfObj::UInt(self as u64)
    }
}

impl Into<PdfObj> for u32 {
    fn into(self) -> PdfObj {
        PdfObj::UInt(self as u64)
    }
}

impl Into<PdfObj> for u64 {
    fn into(self) -> PdfObj {
        PdfObj::UInt(self as u64)
    }
}

impl Into<PdfObj> for f32 {
    fn into(self) -> PdfObj {
        PdfObj::Float(self as f64)
    }
}

impl Into<PdfObj> for f64 {
    fn into(self) -> PdfObj {
        PdfObj::Float(self)
    }
}

impl<T: Into<PdfObj>> Into<PdfObj> for Vec<T> {
    fn into(self) -> PdfObj {
        PdfObj::Array(self.into_iter().map(|e| e.into()).collect())
    }
}
