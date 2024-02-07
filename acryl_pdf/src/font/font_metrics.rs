use acryl_core::math::Pt;

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

macro_rules! unit_fn {
    ($name:ident) => {
        #[inline]
        pub fn $name(&self, font_size: f64) -> Pt {
            Font::unit_to_pt(self.font.units_per_em, self.$name) * font_size
        } 
    }
}

impl<'a> FontMetrics<'a> {

    unit_fn!(ascender);
    unit_fn!(descender);
    unit_fn!(height);
    unit_fn!(leading);
    
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
