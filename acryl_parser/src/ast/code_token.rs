#[derive(Debug)]
pub enum CodeToken<'src> {
    Ident(&'src str),
    Str(&'src str),
    Int(i64),
    Float(f64),
}


impl<'src> CodeToken<'src> {
    pub fn as_ident(&self) -> Option<&str> {
        match self {
            Self::Ident(value) => Some(value),
            _ => None
        }
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(value) => Some(value),
            _ => None
        }
    }

    pub fn as_int(&self) -> Option<&i64> {
        match self {
            Self::Int(value) => Some(value),
            _ => None
        }
    }

    pub fn as_float(&self) -> Option<&f64> {
        match self {
            Self::Float(value) => Some(value),
            _ => None
        }
    }
}