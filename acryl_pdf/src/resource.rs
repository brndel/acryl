use crate::{
    pdf_dict,
    render::{Context, PdfObj, PdfObjRef},
};

pub struct Resources {
    font: PdfObjRef,
}

impl Resources {
    pub fn from(context: &Context) -> Self {
        Self {
            font: context.get_font_obj(),
        }
    }
}

impl Into<PdfObj> for Resources {
    fn into(self) -> PdfObj {
        pdf_dict!(
            "Font" => self.font.into()
        )
    }
}
