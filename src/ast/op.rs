use std::fmt::Display;

#[derive(Debug, PartialEq, Clone)]
pub struct Op<'src>(pub &'src str);

impl<'src> Op<'static> {
    pub const ALLOWED: &'static str = "+-*/!=~^_<>?";

    // Unary
    pub const NOT: Op<'static> = Self("!");
    pub const NEGATIVE: Op<'static> = Self("-");

    // Binary
    pub const MULTIPLY: Op<'static> = Self("*");
    pub const DIVIDE: Op<'static> = Self("/");

    pub const ADD: Op<'static> = Self("+");
    pub const SUBTRACT: Op<'static> = Self("-");

    pub const EQUALS: Op<'static> = Self("==");
    pub const NOT_EQUALS: Op<'static> = Self("!=");
    pub const GREATER: Op<'static> = Self(">");
    pub const GREATER_EQUALS: Op<'static> = Self(">=");
    pub const LESS: Op<'static> = Self("<");
    pub const LESS_EQUALS: Op<'static> = Self("<=");

    // Other
    pub const SET: Op<'static> = Self("=");
    pub const ARROW: Op<'static> = Self("->");

    // Binary priority list
    pub const PRIORITY_HIGH: &'static [Op<'static>; 2] = &[Self::MULTIPLY, Self::DIVIDE];
    pub const PRIORITY_MID: &'static [Op<'static>; 2] = &[Self::ADD, Self::SUBTRACT];
    pub const PRIORITY_LOW: &'static [Op<'static>; 6] = &[
        Self::EQUALS,
        Self::NOT_EQUALS,
        Self::GREATER,
        Self::GREATER_EQUALS,
        Self::LESS,
        Self::LESS_EQUALS,
    ];
}

impl Display for Op<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
