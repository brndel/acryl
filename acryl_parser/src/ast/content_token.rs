use super::CodeToken;

#[derive(Debug)]
pub enum ContentToken<'src> {
    Word(&'src str),
    Fn {
        name: &'src str,
        key: Option<&'src str>,
        arguments: Vec<Argument<'src>>,
        content: Vec<Self>,
    },
}

#[derive(Debug)]
pub enum Argument<'src> {
    Named {
        name: &'src str,
        value: CodeToken<'src>,
    },
    Unnamed(CodeToken<'src>),
}
