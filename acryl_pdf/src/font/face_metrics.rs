use owned_ttf_parser::Face;

use crate::unit::Pt;

#[derive(Debug)]
pub struct FaceMetrics {
    pub(super) ascender: f64,
    pub(super) descender: f64,
    pub(super) leading: f64,
    pub(super) cap_height: f64,
    pub(super) height: f64,
    units_per_em: f64,
}

impl<'a> From<&Face<'a>> for FaceMetrics {
    fn from(value: &Face<'a>) -> Self {
        let ascender = value.ascender();
        Self {
            ascender: ascender as f64,
            descender: value.descender() as f64,
            leading: value.line_gap() as f64,
            cap_height: value.capital_height().unwrap_or(ascender) as f64,
            height: value.height() as f64,
            units_per_em: value.units_per_em() as f64,
        }
    }
}

impl FaceMetrics {
    fn transform(&self, value: f64, font_size: f64) -> Pt {
        Pt((value * font_size) / self.units_per_em)
    }

    pub fn ascender(&self, font_size: f64) -> Pt {
        self.transform(self.ascender, font_size)
    }

    pub fn descender(&self, font_size: f64) -> Pt {
        self.transform(self.descender, font_size)
    }

    pub fn height(&self, font_size: f64) -> Pt {
        self.transform(self.height, font_size)
    }

    pub fn leading(&self, font_size: f64) -> Pt {
        self.transform(self.leading, font_size)
    }

    pub fn sized<'a>(&'a self, font_size: f64) -> SizedFontMetrics<'a> {
        SizedFontMetrics {
            metrics: self,
            font_size,
        }
    }
}

pub struct SizedFontMetrics<'a> {
    metrics: &'a FaceMetrics,
    font_size: f64,
}

impl<'a> SizedFontMetrics<'a> {
    pub fn ascender(&self) -> Pt {
        self.metrics.ascender(self.font_size)
    }

    pub fn descender(&self) -> Pt {
        self.metrics.descender(self.font_size)
    }

    pub fn height(&self) -> Pt {
        self.metrics.height(self.font_size)
    }

    pub fn leading(&self) -> Pt {
        self.metrics.leading(self.font_size)
    }
}
