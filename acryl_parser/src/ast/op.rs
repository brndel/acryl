pub struct Op;

impl Op {
    pub const ALLOWED: &'static str = "+-*/!=~^_<>?#";

    // Unary
    pub const NOT: &'static str = "!";
    pub const NEGATIVE: &'static str = "-";

    // Binary
    pub const MULTIPLY: &'static str = "*";
    pub const DIVIDE: &'static str = "/";

    pub const ADD: &'static str = "+";
    pub const SUBTRACT: &'static str = "-";

    pub const EQUALS: &'static str = "==";
    pub const NOT_EQUALS: &'static str = "!=";
    pub const GREATER: &'static str = ">";
    pub const GREATER_EQUALS: &'static str = ">=";
    pub const LESS: &'static str = "<";
    pub const LESS_EQUALS: &'static str = "<=";

    // Other
    pub const SET: &'static str = "=";
    pub const ARROW: &'static str = "->";

    // Binary priority list
    pub const PRIORITY_HIGH: &'static [&'static str; 2] = &[Self::MULTIPLY, Self::DIVIDE];
    pub const PRIORITY_MID: &'static [&'static str; 2] = &[Self::ADD, Self::SUBTRACT];
    pub const PRIORITY_LOW: &'static [&'static str; 6] = &[
        Self::EQUALS,
        Self::NOT_EQUALS,
        Self::GREATER,
        Self::GREATER_EQUALS,
        Self::LESS,
        Self::LESS_EQUALS,
    ];
}
