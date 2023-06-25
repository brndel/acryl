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

impl TextStream {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    pub fn then(mut self, element: TextStreamElement) -> Self {
        self.elements.push(element);
        self
    }
}

impl Stream<TextStreamElement> for TextStream {
    fn get_start() -> &'static str {
        "BT"
    }

    fn get_end() -> &'static str {
        "BE"
    }

    fn push(&mut self, element: TextStreamElement) {
        self.elements.push(element)
    }
}

impl From<TextStreamElement> for TextStream {
    fn from(value: TextStreamElement) -> Self {
        Self {
            elements: vec![value],
        }
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
            TextStreamElement::Position(v) => (vec![v.x.into(), v.y.into()], "Td"),
            TextStreamElement::NextLine => (vec![], "T*"),
            TextStreamElement::Font(name, size) => (vec![PdfObj::Name(name.into()), size.into()], "Tf"),
            TextStreamElement::CharSpace(v) => (vec![v.into()], "Tc"),
            TextStreamElement::WordSpace(v) => (vec![v.into()], "Tw"),
            TextStreamElement::Scale(v) => (vec![v.into()], "Tz"),
            TextStreamElement::Leading(v) => (vec![v.into()], "TL"),
            TextStreamElement::RenderMode(v) => (vec![(v as u8).into()], "Tr"),
            TextStreamElement::Rise(v) => (vec![v.into()], "Tj"),
            TextStreamElement::Text(text) => (vec![PdfObj::StringLiteral(text)], "Tj"),
        }
    }
}


impl StreamElement<TextStream> for TextStreamElement {
}