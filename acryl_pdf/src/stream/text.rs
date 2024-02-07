use acryl_core::math::{PdfCoords, Pt, Vector2};

use crate::data::PdfObj;

use super::StreamInstruction;

#[derive(Clone)]
pub enum TextControl {
    Begin,
    End,
}

pub enum TextStreamElement {
    Position(Vector2<Pt, PdfCoords>),
    NextLine,
    Font(String, Pt),
    CharSpace(Pt),
    WordSpace(Pt),
    Scale(u16),
    Leading(Pt),
    RenderMode(RenderMode),
    Rise(Pt),
    Text(Vec<u8>),
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

impl From<TextControl> for StreamInstruction {
    fn from(value: TextControl) -> Self {
        match value {
            TextControl::Begin => (vec![], "BT"),
            TextControl::End => (vec![], "ET"),
        }
    }
}

impl From<TextStreamElement> for StreamInstruction {
    fn from(value: TextStreamElement) -> Self {
        match value {
            TextStreamElement::Position(v) => (vec![v.x.into(), v.y.into()], "Td"),
            TextStreamElement::NextLine => (vec![], "T*"),
            TextStreamElement::Font(name, size) => {
                (vec![PdfObj::name(name), size.into()], "Tf")
            }
            TextStreamElement::CharSpace(v) => (vec![v.into()], "Tc"),
            TextStreamElement::WordSpace(v) => (vec![v.into()], "Tw"),
            TextStreamElement::Scale(v) => (vec![v.into()], "Tz"),
            TextStreamElement::Leading(v) => (vec![v.into()], "TL"),
            TextStreamElement::RenderMode(v) => (vec![(v as u8).into()], "Tr"),
            TextStreamElement::Rise(v) => (vec![v.into()], "Ts"),
            TextStreamElement::Text(text) => (vec![PdfObj::HexString(text)], "Tj"),
        }
    }
}
