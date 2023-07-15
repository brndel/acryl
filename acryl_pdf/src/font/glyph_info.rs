use crate::{unit::Pt, Vector2};

use super::ExternalFont;

pub struct GlyphInfo<'a> {
    font: &'a ExternalFont,
    id: u16,
    name: &'a str,
    advance: Vector2<u16>,
}

impl<'a> GlyphInfo<'a> {
    pub(crate) fn new(
        font: &'a ExternalFont,
        id: u16,
        name: &'a str,
        advance: Vector2<u16>,
    ) -> Self {
        Self {
            font,
            id,
            name,
            advance,
        }
    }

    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.name
    }

    #[inline]
    pub fn advance(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: self.font.unit_to_pt(self.advance.x, font_size),
            y: self.font.unit_to_pt(self.advance.y, font_size),
        }
    }
}
