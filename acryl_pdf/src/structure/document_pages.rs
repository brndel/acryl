use crate::{
    data::{PdfObj, PdfObjRef},
    pdf_dict,
    write::{PdfWriter, WritePdf},
};

use super::Page;

pub struct DocumentPages {
    pages: Vec<Page>,
}

impl DocumentPages {
    pub fn new(pages: Vec<Page>) -> Self {
        Self { pages }
    }
}

impl WritePdf for DocumentPages {
    fn write(self, writer: &mut PdfWriter) -> PdfObjRef {
        writer.add_reserved(|writer| {
            let mut kids = Vec::new();
    
            for page in self.pages {
                let id = page.write(writer);
                kids.push(id);
            }
    
            pdf_dict!(
                "Type" => PdfObj::name("Pages"),
                "Count" => kids.len(),
                "Kids" => kids,
            )
        })
    }
}