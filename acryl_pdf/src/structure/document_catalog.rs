use crate::{
    data::{PdfObj, PdfObjRef},
    pdf_dict,
    write::{PdfWriter, PdfWriterDefaultData, WritePdf},
};

use super::{DocumentPages, Page};

pub struct DocumentCatalog {
    pages: DocumentPages,
}

impl DocumentCatalog {
    pub fn new(pages: Vec<Page>) -> Self {
        Self {
            pages: DocumentPages::new(pages),
        }
    }
}

impl WritePdf<PdfWriterDefaultData> for DocumentCatalog {
    fn write(self, writer: &mut PdfWriter) -> PdfObjRef {
        writer.add_reserved(|writer| {
            let pages = self.pages.write(writer);
    
            pdf_dict!(
                "Type" => PdfObj::name("Catalog"),
                "Pages" => pages,
            )
        })
    }
}
