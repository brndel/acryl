use crate::{unit::Pt, Vector2};

use super::Font;

pub struct GlyphInfo<'a> {
    pub(super) font: &'a Font,
    pub(super) id: u16,
    pub(super) advance: Vector2<u16>,
    pub(super) size: Vector2<u16>,
}

impl<'a> GlyphInfo<'a> {
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn advance(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: self.font.unit_to_pt(self.advance.x, font_size),
            y: self.font.unit_to_pt(self.advance.y, font_size),
        }
    }

    #[inline]
    pub fn size(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: self.font.unit_to_pt(self.size.x, font_size),
            y: self.font.unit_to_pt(self.size.y, font_size),
        }
    }
}
