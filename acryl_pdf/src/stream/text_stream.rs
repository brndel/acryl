use crate::{render::PdfObj, unit::Pt, Vector2};

use super::{Stream, StreamElement, StreamInstruction};

#[derive(Clone)]
pub struct TextStream {
    elements: Vec<TextStreamElement>,
}

#[derive(Clone)]
pub enum TextStreamElement {
    Position(Vector2<Pt>),
    NextLine,
    Font(String, Pt),
    CharSpace(Pt),
    WordSpace(Pt),
    Scale(Pt),
    Leading(Pt),
    RenderMode(RenderMode),
    Rise(Pt),
    Text(String),
}

#[repr(u8)]
#[derive(Clone)]
pub enum RenderMode {
    Fill = 0,
    Stroke,
    FillStroke,
    Invisible,
    FillClip,
    StrokeClip,
    FillStrokeClip,
    Clip,
}

impl Into<PdfObj> for RenderMode {
    fn into(self) -> PdfObj {
        PdfObj::Int(self as i64)
    }
}

impl TextStream {
    pub fn new() -> Self {
        Self { elements: Vec::new() }
    }

    pub fn then(mut self, element: TextStreamElement) -> Self {
        self.elements.push(element);
        self
    }
}

impl Stream<TextStreamElement> for TextStream {
    fn get_name() -> &'static str {
        "T"
    }

    fn push(&mut self, element: TextStreamElement) {
        self.elements.push(element)
    }

}

impl From<TextStreamElement> for TextStream {
    fn from(value: TextStreamElement) -> Self {
        Self { elements: vec![value] }
    }
}

impl Into<Vec<TextStreamElement>> for TextStream {
    fn into(self) -> Vec<TextStreamElement> {
        self.elements
    }
}

impl Into<StreamInstruction> for TextStreamElement {
    fn into(self) -> StreamInstruction {
        match self {
            TextStreamElement::Position(v) => (vec![v.x.into(), v.y.into()], "d"),
            TextStreamElement::NextLine => (vec![], "*"),
            TextStreamElement::Font(name, size) => (vec![PdfObj::Name(name), size.into()], "f"),
            TextStreamElement::CharSpace(v) => (vec![v.into()], "c"),
            TextStreamElement::WordSpace(v) => (vec![v.into()], "w"),
            TextStreamElement::Scale(v) => (vec![v.into()], "z"),
            TextStreamElement::Leading(v) => (vec![v.into()], "L"),
            TextStreamElement::RenderMode(v) => (vec![v.into()], "r"),
            TextStreamElement::Rise(v) => (vec![v.into()], "j"),
            TextStreamElement::Text(text) => (vec![PdfObj::StringLiteral(text)], "j"),
        }
    }
}


impl StreamElement<TextStream> for TextStreamElement {
    fn get_prefix() -> &'static str {
        "T"
    }
}