
#[derive(Clone, Debug, PartialEq)]
pub enum Token<'src> {
    Word(&'src str),
    Num(&'src str),
    Str(&'src str),

    Op(&'src str),
    Ctrl(char),
    Escape,
}