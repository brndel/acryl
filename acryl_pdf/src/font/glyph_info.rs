use acryl_core::math::{Pt, Vector2};

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
        Vector2::new(
            Font::unit_to_pt(self.units_per_em, self.advance.x),
            Font::unit_to_pt(self.units_per_em, self.advance.y),
        )
        .scale(font_size)
    }

    #[inline]
    pub fn size(&self, font_size: f64) -> Vector2<Pt> {
        Vector2::new(
            Font::unit_to_pt(self.units_per_em, self.size.x),
            Font::unit_to_pt(self.units_per_em, self.size.y),
        )
        .scale(font_size)
    }
}
