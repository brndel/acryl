use crate::{data::{PdfObj, PdfObjRef}, pdf_dict
};

pub struct Resources {
    font_container: PdfObjRef,
}

impl Resources {
    pub fn new(font_container: PdfObjRef) -> Self {
        Self { font_container }
    }
}

impl<'a> From<Resources> for PdfObj {
    fn from(value: Resources) -> Self {
        pdf_dict!(
            "Font" => value.font_container
        )
    }
}
