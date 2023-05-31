use std::fmt::Display;

use super::stream::PdfStream;

pub enum PdfObj {
    Null,
    Bool(bool),
    Int(i64),
    UInt(u64),
    StringLiteral(String),
    Name(&'static str),
    Array(Vec<Self>),
    Dict(Vec<(&'static str, Self)>),
    Stream(PdfStream),
    Refernce(u64, u64),
}

impl Display for PdfObj {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PdfObj::Null => write!(f, "null"),
            PdfObj::Bool(value) => write!(f, "{}", if *value { "true" } else { "false" }),
            PdfObj::Int(value) => write!(f, "{}", value),
            PdfObj::UInt(value) => write!(f, "{}", value),
            PdfObj::StringLiteral(value) => {
                write!(f, "({})", value.replace('(', "\\(").replace(')', "\\)"))
            }
            PdfObj::Name(value) => write!(f, "\\{}", value),
            PdfObj::Array(values) => {
                write!(f, "[")?;

                for value in values {
                    write!(f, "{}", value);
                }

                write!(f, "]")
            }
            PdfObj::Dict(fields) => {
                writeln!(f, "<<")?;

                for (name, value) in fields {
                    writeln!(f, "\\{} {}", name, value)?;
                }

                write!(f, ">>")
            }
            PdfObj::Stream(stream) => {
                let stream_content = stream.build();
                let dict = PdfObj::Dict(vec![
                    ("Length", stream_content.len().into())
                ]);
                writeln!(f, "{}", dict)?;
                writeln!(f, "stream")?;

                writeln!(f, "{}", stream_content)?;

                writeln!(f, "endstream")
            }
            PdfObj::Refernce(id, generation) => write!(f, "{} {} R", id, generation),
        }
    }
}

impl Into<PdfObj> for u64 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for usize {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for i64 {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl Into<PdfObj> for isize {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl<T: Into<PdfObj>> Into<PdfObj> for Vec<T> {
    fn into(self) -> PdfObj {
        PdfObj::Array(self.into_iter().map(|e| e.into()).collect())
    }
}