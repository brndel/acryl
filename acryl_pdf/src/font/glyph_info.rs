use crate::{unit::Pt, Vector2};

use super::ExternalFont;

pub struct GlyphInfo<'a> {
    pub(super) font: &'a ExternalFont,
    pub id: u16,
    pub name: &'a str,
    pub(crate) advance: Vector2<u16>,
}

impl<'a> GlyphInfo<'a> {
    pub fn advance(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: self.font.unit_to_pt(self.advance.x, font_size),
            y: self.font.unit_to_pt(self.advance.y, font_size),
        }
    }
}
