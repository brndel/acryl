pub struct Op;

impl Op {
    pub const ALLOWED: &str = "+-*/!=~^_<>?#";

    // Unary
    pub const NOT: &str = "!";
    pub const NEGATIVE: &str = "-";

    // Binary
    pub const MULTIPLY: &str = "*";
    pub const DIVIDE: &str = "/";

    pub const ADD: &str = "+";
    pub const SUBTRACT: &str = "-";

    pub const EQUALS: &str = "==";
    pub const NOT_EQUALS: &str = "!=";
    pub const GREATER: &str = ">";
    pub const GREATER_EQUALS: &str = ">=";
    pub const LESS: &str = "<";
    pub const LESS_EQUALS: &str = "<=";

    // Other
    pub const SET: &str = "=";
    pub const ARROW: &str = "->";

    // Binary priority list
    pub const PRIORITY_HIGH: &[&'static str; 2] = &[Self::MULTIPLY, Self::DIVIDE];
    pub const PRIORITY_MID: &[&'static str; 2] = &[Self::ADD, Self::SUBTRACT];
    pub const PRIORITY_LOW: &[&'static str; 6] = &[
        Self::EQUALS,
        Self::NOT_EQUALS,
        Self::GREATER,
        Self::GREATER_EQUALS,
        Self::LESS,
        Self::LESS_EQUALS,
    ];
}
