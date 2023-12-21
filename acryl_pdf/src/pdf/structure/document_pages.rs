use crate::{writer::PdfWriter, pdf::{PdfObjRef, PdfObj}, pdf_dict};

use super::Page;


pub struct DocumentPages {
    pages: Vec<Page>
}

impl DocumentPages {
    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages }
    }

    pub fn render<T: PdfWriter>(self, writer: &mut T, font_container: PdfObjRef) -> PdfObjRef {
        let obj_ref = writer.reserve();

        let mut kids = Vec::new();

        for page in self.pages {
            let id = page.render(writer, obj_ref, font_container);
            kids.push(id);
        }

        let obj = pdf_dict!(
            "Type" => PdfObj::Name("Pages".into()),
            "Count" => kids.len().into(),
            "Kids" => kids.into(),
        );

        writer.insert(obj_ref, obj);

        obj_ref
    }
}
