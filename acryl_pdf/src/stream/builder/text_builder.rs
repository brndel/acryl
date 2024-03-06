use std::rc::Rc;

use acryl_core::math::{AcrylCoords, Pt, Vector2};

use crate::{
    font::{Font, WordLayout},
    resource::resource_manager::ResourceRef,
    stream::text::{TextControl, TextStreamElement},
};

use super::StreamBuilder;

pub struct TextBuilder<'builder, 'page> {
    builder: &'builder mut StreamBuilder<'page>,
    font: Rc<Font>,
    font_size: f64,
}

impl<'builder, 'page> TextBuilder<'builder, 'page> {
    pub fn new(
        builder: &'builder mut StreamBuilder<'page>,
        font_ref: &ResourceRef<Font>,
        font_size: f64,
    ) -> Self {
        builder.push(TextControl::Begin);
        builder.push(TextStreamElement::Font(
            font_ref.name().to_owned(),
            Pt(font_size),
        ));
        Self {
            builder,
            font: font_ref.data_rc(),
            font_size,
        }
    }

    pub fn set_position(&mut self, mut position: Vector2<Pt, AcrylCoords>) {
        position.y += self.font.ascender(self.font_size);

        let position = self.builder.transform(position);

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

    pub fn draw_word(&mut self, word: &WordLayout) {
        let mut bytes = Vec::new();

        for glyph in word.glyphs() {
            let gid_bytes = glyph.glyph_id().0.to_be_bytes();

            bytes.append(&mut gid_bytes.to_vec());
        }

        self.builder.push(TextStreamElement::Text(bytes))
    }
}

impl Drop for TextBuilder<'_, '_> {
    fn drop(&mut self) {
        self.builder.push(TextControl::End);
    }
}
