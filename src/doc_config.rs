use acryl_core::math::{Pt, Vector2};
use acryl_parser::file::DocFileHeader;
use acryl_pdf::structure::DocumentInfo;

use crate::util::page_size::PageSize;

#[derive(Debug)]
pub struct DocumentConfig {
    pub info: DocumentInfo,
    pub default_page_size: Vector2<Pt>,
}

impl TryFrom<&DocFileHeader<'_>> for DocumentConfig {
    type Error = &'static str;

    fn try_from(value: &DocFileHeader) -> Result<Self, Self::Error> {
        fn some_to_result<T, R, E, F: Fn(T) -> Result<R, E>>(
            value: Option<T>,
            f: F,
        ) -> Result<Option<R>, E> {
            match value {
                Some(value) => f(value).map(Some),
                None => Ok(None),
            }
        }

        let title = some_to_result(value.get("title"), |token| {
            token
                .as_str()
                .ok_or("'title' needs to be of type str'")
                .map(ToOwned::to_owned)
        })?;

        let author = some_to_result(value.get("author"), |token| {
            token
                .as_str()
                .ok_or("'author' needs to be of type str'")
                .map(ToOwned::to_owned)
        })?;

        let subject = some_to_result(value.get("subject"), |token| {
            token
                .as_str()
                .ok_or("'subject' needs to be of type str'")
                .map(ToOwned::to_owned)
        })?;

        let page_size = match value.get("pageSize") {
            Some(size) => size
                .as_ident()
                .ok_or("'pageSize' needs to be of type ident")?
                .parse()
                .map_err(|_| "Unknow Page Size")?,
            None => PageSize::default(),
        };

        Ok(DocumentConfig {
            info: DocumentInfo {
                title,
                author,
                subject,
            },
            default_page_size: page_size.get_size(),
        })
    }
}
