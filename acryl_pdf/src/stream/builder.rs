use std::rc::Rc;

use crate::{
    font::{Font, FontRef},
    unit::Pt,
    util::{Area, CoordinateTransformer},
    Page, Vector2,
};

use super::{
    color::{Color, ColorOperation},
    graphics_state::GraphicsState,
    path_construction::PathConstruction,
    path_painting::PathPainting,
    text::{TextControl, TextStreamElement},
    Stream, StreamInstruction,
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

    pub fn get_area(&self) -> &Area<Pt> {
        &self.page.area()
    }

    pub fn render(self) {
        self.page.push(Stream::new(self.instructions))
    }

    fn push<T: Into<StreamInstruction>>(&mut self, instr: T) {
        self.instructions.push(instr.into())
    }

    pub fn text<'b>(&'b mut self, font_ref: &FontRef, size: f64) -> TextStreambuilder<'b, 'a> {
        TextStreambuilder::new(self, font_ref, size)
    }

    pub fn draw_rect(&mut self, rect: Area<Pt>, color: Color) {
        let rect = self.page.transform(rect);

        self.push(GraphicsState::SaveState);
        self.push(PathConstruction::Rect(rect));
        self.push(ColorOperation::FillColor(color));
        self.push(PathPainting::Fill(super::path_painting::FillRule::EvenOdd));
        self.push(GraphicsState::RestoreState);
    }
}

pub struct TextStreambuilder<'a, 'b> {
    builder: &'a mut Streambuilder<'b>,
    font: Rc<Font>,
    font_size: f64,
}

impl<'a, 'b> TextStreambuilder<'a, 'b> {
    pub fn new(builder: &'a mut Streambuilder<'b>, font_ref: &FontRef, font_size: f64) -> Self {
        builder.push(TextControl::Begin);
        builder.push(TextStreamElement::Font(
            font_ref.name().to_owned(),
            Pt(font_size),
        ));
        Self {
            builder,
            font: font_ref.1.clone(),
            font_size,
        }
    }

    pub fn set_position(&mut self, mut position: Vector2<Pt>) {
        position.y += self.font.metrics().ascender(self.font_size);

        position = self.builder.page.transform(position);

        self.builder.push(TextStreamElement::Position(position))
    }

    pub fn set_scale(&mut self, scale: u16) {
        self.builder.push(TextStreamElement::Scale(scale))
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

    /**
     * The spacing between baselines of consecutive lines of text.
     * Default value: 0
     */
    pub fn set_leading(&mut self, leading: Pt) {
        self.builder.push(TextStreamElement::Leading(leading))
    }

    pub fn set_rise(&mut self, rise: Pt) {
        self.builder.push(TextStreamElement::Rise(rise))
    }

    pub fn next_line(&mut self) {
        self.builder.push(TextStreamElement::NextLine)
    }

    pub fn draw_text<T: Into<String>>(&mut self, text: T) {
        let mut bytes = Vec::new();

        for c in text.into().chars() {
            if let Some(gid) = self.font.as_ref().get_char_id(c) {
                let gid_bytes = gid.to_be_bytes();

                bytes.append(&mut gid_bytes.to_vec());
            } else {
                dbg!("Invalid char '{c}'");
            }
        }

        self.builder.push(TextStreamElement::Text(bytes))
    }
}

impl<'a, 'b> Drop for TextStreambuilder<'a, 'b> {
    fn drop(&mut self) {
        self.builder.push(TextControl::End);
    }
}
