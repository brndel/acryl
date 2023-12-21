use acryl_core::{unit::Pt, Area, Vector2};

use crate::{
    pdf::{PdfObj, PdfObjRef},
    pdf_dict,
    stream::Stream,
    writer::PdfWriter, util::CoordinateTransformer,
};

use super::Resources;

pub struct Page {
    area: Area<Pt>,
    content: Vec<PdfObj>,
}

impl Page {
    pub fn new(size: Vector2<Pt>) -> Self {
        Self {
            area: Area::from_size(size),
            content: Vec::default(),
        }
    }

    pub fn area(&self) -> &Area<Pt> {
        &self.area
    }

    pub fn render<T: PdfWriter>(self, writer: &mut T, parent: PdfObjRef, font_container: PdfObjRef) -> PdfObjRef {
        let mut content_refs = Vec::<PdfObjRef>::new();

        for obj in self.content {
            let obj_ref = obj.add_to(writer);
            content_refs.push(obj_ref);
        }

        let obj = pdf_dict!(
            "Type" => PdfObj::Name("Page".into()),
            "Parent" => parent.into(),
            "MediaBox" => self.area.into(),
            "Contents" => content_refs.into(),
            "Resources" => Resources::new(font_container).into()
        );

        writer.add(obj)
    }

    pub fn add_stream(&mut self, stream: Stream) {
        match stream.render() {
            Ok(content) => self.content.push(PdfObj::Stream(content.into())),
            Err(err) => panic!("could not render stream {:?}", err),
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
