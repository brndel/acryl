use crate::pdf_dict;

use crate::pdf::PdfObj;

#[derive(Debug)]
pub struct DocumentInfo {
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
}

impl From<DocumentInfo> for PdfObj {
    fn from(value: DocumentInfo) -> Self {
        macro_rules! literal {
            ($name:ident) => {
                value.$name.map(|s| PdfObj::StringLiteral(s.into())).into()
            };
        }
        pdf_dict!(
            "Title" => literal!(title),
            "Author" => literal!(author),
            "Subject" => literal!(subject),
            "Creator" => PdfObj::StringLiteral("Acryl".into()),
        )
    }
}