use std::fmt::Display;

/**
    A literal value
*/
#[derive(Debug)]
pub enum Literal<'src> {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(&'src str),
}

impl<'src> Literal<'src> {
    pub fn parse_num(num: &'src str) -> Self {
        if num.contains('.') {
            Self::Float(num.parse().unwrap())
        } else {
            Self::Int(num.parse().unwrap())
        }
    }
}


impl Display for Literal<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal::Null => write!(f, "NULL"),
            Literal::Bool(v) => write!(f, "{}", v),
            Literal::Int(v) => write!(f, "{}", v),
            Literal::Float(v) => write!(f, "{}", v),
            Literal::Str(v) => write!(f, "'{}'", v),
        }
    }
}