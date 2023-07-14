use crate::{
    unit::Pt,
    util::{Area, CoordinateTransformer},
    Page, Vector2, font::FontRef,
};

use super::{
    color::{Color, ColorOperation},
    path_construction::PathConstruction,
    path_painting::PathPainting,
    text::{TextControl, TextStreamElement},
    Stream, StreamInstruction, graphics_state::GraphicsState,
};

pub struct Streambuilder<'a> {
    page: &'a mut Page,
    instructions: Vec<StreamInstruction>,
}

impl<'a> Streambuilder<'a> {
    pub fn new(page: &'a mut Page) -> Self {
        Self {
            page,
            instructions: Vec::new(),
        }
    }

    pub fn render(self) {
        self.page.push(Stream::new(self.instructions))
    }

    fn push<T: Into<StreamInstruction>>(&mut self, instr: T) {
        self.instructions.push(instr.into())
    }

    pub fn text<'b>(&'b mut self, font: FontRef, size: f64) -> TextStreambuilder<'b, 'a> {
        TextStreambuilder::new(self, font, size)
    }

    pub fn draw_rect(&mut self, rect: Area<Pt>, color: Color) {
        let rect = self.page.transform(rect);

        self.push(GraphicsState::SaveState);
        self.push(PathConstruction::Rect(rect));
        self.push(ColorOperation::FillColor(color));
        self.push(PathPainting::Fill(
            super::path_painting::FillRule::EvenOdd,
        ));
        self.push(GraphicsState::RestoreState);

    }
}

pub struct TextStreambuilder<'a, 'b> {
    builder: &'a mut Streambuilder<'b>,
}

impl<'a, 'b> TextStreambuilder<'a, 'b> {
    pub fn new(builder: &'a mut Streambuilder<'b>, font: FontRef, size: f64) -> Self {
        builder.push(TextControl::Begin);
        // TODO: add proper fonts and remove this default value
        builder.push(TextStreamElement::Font(font.as_ref().to_owned(), Pt(size)));
        Self { builder }
    }

    pub fn set_position(&mut self, position: Vector2<Pt>) {
        self.builder.push(TextStreamElement::Position(
            self.builder.page.transform(position),
        ))
    }

    pub fn set_scale(&mut self, scale: u16) {
        self.builder.push(TextStreamElement::Scale(
            scale
        ))
    }

    pub fn set_line_height(&mut self, height: Pt) {
        self.builder.push(TextStreamElement::Leading(height))
    }

    pub fn set_char_spacing(&mut self, spacing: Pt) {
        self.builder.push(TextStreamElement::CharSpace(spacing))
    }

    pub fn set_word_spacing(&mut self, spacing: Pt) {
        self.builder.push(TextStreamElement::WordSpace(spacing))
    }

    pub fn set_leading(&mut self, leading: Pt) {
        self.builder.push(TextStreamElement::Leading(leading))
    }

    pub fn set_rise(&mut self, rise: Pt) {
        self.builder.push(TextStreamElement::Rise(rise))
    }

    pub fn draw_text<T: ToString>(&mut self, text: T) {
        self.builder.push(TextStreamElement::Text(text.to_string()))
    }
}

impl<'a, 'b> Drop for TextStreambuilder<'a, 'b> {
    fn drop(&mut self) {
        self.builder.push(TextControl::End);
    }
}
