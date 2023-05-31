use crate::{render::{Context, PdfObjRef}, util::Area};

use super::{PageElement, word::Word};



pub(crate) struct Flow {
    elements: Vec<Word>
}

impl PageElement for Flow {
    fn render(&self, context: &Context, area: &Area) -> PdfObjRef {
        todo!()
    }
}