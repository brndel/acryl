#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    True,
    False,
    If,
    Else,
    Let,
    Fn,
    Struct,
    In,
    After,
    Return,
    Null,
}

impl Keyword {
    pub fn parse(value: &str) -> Option<Self> {
        use Keyword::*;
        let keyword = match value {
            "true" => True,
            "false" => False,
            "if" => If,
            "else" => Else,
            "let" => Let,
            "fn" => Fn,
            "struct" => Struct,
            "in" => In,
            "after" => After,
            "return" => Return,
            "null" => Null,
            _ => return None,
        };

        Some(keyword)
    }
}
