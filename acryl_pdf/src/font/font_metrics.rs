use crate::unit::Pt;

use super::Font;

#[derive(Debug)]
pub struct FontMetrics<'a> {
    pub(super) font: &'a Font,
    pub(super) ascender: i16,
    pub(super) descender: i16,
    pub(super) leading: i16,
    pub(super) cap_height: i16,
    pub(super) height: i16,
}

impl<'a> From<&'a Font> for FontMetrics<'a> {
    fn from(value: &'a Font) -> Self {
        let face = value.face();
        let ascender = face.ascender();
        Self {
            font: value,
            ascender,
            descender: face.descender(),
            leading: face.line_gap(),
            cap_height: face.capital_height().unwrap_or(ascender),
            height: face.height(),
        }
    }
}

impl<'a> FontMetrics<'a> {
    #[inline]
    pub fn ascender(&self, font_size: f64) -> Pt {
        self.font.unit_to_pt(self.ascender, font_size)
    }

    #[inline]
    pub fn descender(&self, font_size: f64) -> Pt {
        self.font.unit_to_pt(self.descender, font_size)
    }

    #[inline]
    pub fn height(&self, font_size: f64) -> Pt {
        self.font.unit_to_pt(self.height, font_size)
    }

    #[inline]
    pub fn leading(&self, font_size: f64) -> Pt {
        self.font.unit_to_pt(self.leading, font_size)
    }

    #[inline]
    pub fn sized(self, font_size: f64) -> SizedFontMetrics<'a> {
        SizedFontMetrics {
            metrics: self,
            font_size,
        }
    }
}

pub struct SizedFontMetrics<'a> {
    metrics: FontMetrics<'a>,
    font_size: f64,
}

impl<'a> SizedFontMetrics<'a> {
    #[inline]
    pub fn ascender(&self) -> Pt {
        self.metrics.ascender(self.font_size)
    }

    #[inline]
    pub fn descender(&self) -> Pt {
        self.metrics.descender(self.font_size)
    }

    #[inline]
    pub fn height(&self) -> Pt {
        self.metrics.height(self.font_size)
    }

    #[inline]
    pub fn leading(&self) -> Pt {
        self.metrics.leading(self.font_size)
    }
}
