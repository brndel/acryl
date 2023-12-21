use acryl_core::{unit::Pt, Vector2};

use super::Font;

#[derive(Debug, Clone)]
pub struct GlyphInfo {
    pub(super) id: u16,
    pub(super) ch: char,
    pub(super) advance: Vector2<u16>,
    pub(super) size: Vector2<u16>,
    pub(super) units_per_em: u16,
}

impl GlyphInfo {
    #[inline]
    pub fn id(&self) -> u16 {
        self.id
    }

    #[inline]
    pub fn advance(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: Font::unit_to_pt(self.units_per_em, self.advance.x),
            y: Font::unit_to_pt(self.units_per_em, self.advance.y),
        }
        .scale(font_size)
    }

    #[inline]
    pub fn size(&self, font_size: f64) -> Vector2<Pt> {
        Vector2 {
            x: Font::unit_to_pt(self.units_per_em, self.size.x),
            y: Font::unit_to_pt(self.units_per_em, self.size.y),
        }
        .scale(font_size)
    }
}
