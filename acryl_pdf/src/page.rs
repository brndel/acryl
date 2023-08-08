use acryl_core::{unit::Pt, Vector2, Area};

use crate::{
    pdf_dict,
    render::{Context, PdfObj, PdfObjRef},
    resource::Resources,
    stream::Stream,
    util::CoordinateTransformer,
};

pub struct Page {
    area: Area<Pt>,
    content: Vec<PdfObj>,
}

impl Page {
    pub fn new(area: Area<Pt>) -> Self {
        Self {
            area,
            content: Vec::default(),
        }
    }

    pub fn area(&self) -> &Area<Pt> {
        &self.area
    }

    pub fn render(self, context: &mut Context, parent: PdfObjRef) -> PdfObjRef {
        let mut content_refs = Vec::<PdfObjRef>::new();

        for element in self.content {
            content_refs.push(context.add(element));
        }

        let obj = pdf_dict!(
            "Type" => PdfObj::Name("Page".into()),
            "Parent" => parent.into(),
            "MediaBox" => self.area.into(),
            "Contents" => content_refs.into(),
            "Resources" => Resources::from(context).into()
        );

        context.add(obj)
    }

    pub fn push(&mut self, stream: Stream) {
        if let Ok(content) = stream.render() {
            self.content.push(PdfObj::Stream(content))
        }
    }
}

impl CoordinateTransformer<Vector2<Pt>> for Page {
    fn transform(&self, value: Vector2<Pt>) -> Vector2<Pt> {
        self.area.transform(value)
    }
}

impl CoordinateTransformer<Area<Pt>> for Page {
    fn transform(&self, value: Area<Pt>) -> Area<Pt> {
        self.area.transform(value)
    }
}
