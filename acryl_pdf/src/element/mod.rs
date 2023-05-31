use crate::{render::{Context, PdfObjRef}, util::Area};

mod flow;
mod word;

pub(crate) trait PageElement {
    fn render(&self, context: &Context, area: &Area) -> PdfObjRef;
}