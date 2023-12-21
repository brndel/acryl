use crate::{writer::PdfWriter, pdf::{PdfObjRef, PdfObj}, pdf_dict};

use super::{DocumentPages, Page};

pub struct DocumentCatalog {
    pages: DocumentPages,
}

impl DocumentCatalog {

    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages: DocumentPages::new(pages) }
    }

    pub fn render<T: PdfWriter>(self, writer: &mut T, font_container: PdfObjRef) -> PdfObjRef {
        let pages = self.pages.render(writer, font_container);

        pdf_dict!(
            "Type" => PdfObj::Name("Catalog".into()),
            "Pages" => pages.into(),
        )
        .add_to(writer)
    }
}
