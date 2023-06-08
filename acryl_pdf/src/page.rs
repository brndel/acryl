use crate::{render::{PdfObjRef, Context, PdfObj}, util::Area, unit::Pt};

pub struct Page {
    area: Area<Pt>,
}

impl Page {

    pub fn new(area: Area<Pt>) -> Self {
        Self { area}
    }

    pub fn render(&self, context: &mut Context, parent: PdfObjRef) -> PdfObjRef {

        let content = Vec::<PdfObjRef>::new();

        let obj = PdfObj::Dict(vec![
            ("Type", PdfObj::Name("Page".into())),
            ("Parent", parent.into()),
            ("MediaBox", self.area.clone().into()),
            ("Contents", content.into()),
        ]);

        context.add(obj)
    }
}
