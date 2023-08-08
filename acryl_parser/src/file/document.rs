use std::collections::BTreeMap;

use crate::ast::{CodeToken, ContentToken};

#[derive(Debug)]
pub struct DocFile<'src> {
    header: DocFileHeader<'src>,
    content: DocFileContent<'src>,
}

#[derive(Debug)]
pub struct DocFileHeader<'src> {
    fields: BTreeMap<&'src str, CodeToken<'src>>,
}

#[derive(Debug)]
pub struct DocFileContent<'src> {
    tokens: Vec<ContentToken<'src>>,
}

impl<'src> DocFile<'src> {
    pub(crate) fn new(
        header: BTreeMap<&'src str, CodeToken<'src>>,
        tokens: Vec<ContentToken<'src>>,
    ) -> Self {
        Self {
            header: DocFileHeader { fields: header },
            content: DocFileContent { tokens },
        }
    }

    pub fn header(&self) -> &DocFileHeader<'src> {
        &self.header
    }

    pub fn content(&self) -> &DocFileContent<'src> {
        &self.content
    }
}


impl<'src> DocFileHeader<'src> {
    pub fn get(&self, name: &str) -> Option<&CodeToken<'src>> {
        self.fields.get(name)
    }
}

impl<'src> DocFileContent<'src> {
    pub fn tokens(&self) -> &[ContentToken<'src>] {
        &self.tokens
    }
}