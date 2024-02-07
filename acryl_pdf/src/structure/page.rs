use acryl_core::math::{AcrylCoords, Area, PdfCoords, Pt, Vector2};

use crate::{
    data::{PdfObj, PdfObjRef},
    pdf_dict,
    stream::Stream,
    write::{PdfWriter, WritePdf}, util::CoordinateTransformer,
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

    pub fn add_stream(&mut self, stream: Stream) {
        match stream.render() {
            Ok(content) => self.content.push(PdfObj::Stream(content.into())),
            Err(err) => panic!("could not render stream {:?}", err),
        }
    }
}

impl WritePdf for Page {
    fn write(self, writer: &mut PdfWriter) -> PdfObjRef {
        let mut content_refs = Vec::<PdfObjRef>::new();
    
        for obj in self.content {
            let obj_ref = writer.add(obj);
            content_refs.push(obj_ref);
        }

        pdf_dict!(
            "Type" => PdfObj::name("Page"),
            "Parent" => writer.parent(),
            "MediaBox" => self.area.clone().transform(self.area),
            "Contents" => content_refs,
            "Resources" => Resources::new(writer.font_container())
        ).add_to(writer)
    }
}

impl CoordinateTransformer<Vector2<Pt, AcrylCoords>, Vector2<Pt, PdfCoords>> for Page {
    fn transform(&self, value: Vector2<Pt, AcrylCoords>) -> Vector2<Pt, PdfCoords> {
        self.area.transform(value)
    }
}

impl CoordinateTransformer<Area<Pt, AcrylCoords>, Area<Pt, PdfCoords>> for Page {
    fn transform(&self, value: Area<Pt, AcrylCoords>) -> Area<Pt, PdfCoords> {
        self.area.transform(value)
    }
}
