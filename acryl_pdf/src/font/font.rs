use std::fmt::Debug;
use std::{fs, io, path::Path};

use acryl_core::math::Pt;
use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
use owned_ttf_parser::{AsFaceRef, Face, FaceParsingError, GlyphId, OwnedFace};

use super::pdf_font::PdfFont;

#[derive(Debug)]
pub enum FontLoadError {
    File(io::Error),
    Parse(FaceParsingError),
}

pub struct Font {
    face: OwnedFace,
}

impl Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font").field("face", &"[...]").finish()
    }
}

impl Font {
    pub const DEFAULT_GLYPH_UNITS: u16 = 1000;

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, FontLoadError> {
        let data = fs::read(path).map_err(|e| FontLoadError::File(e))?;

        let face = OwnedFace::from_vec(data, 0).map_err(FontLoadError::Parse)?;

        Ok(Self { face })
    }

    pub fn name(&self) -> String {
        self.face
            .as_face_ref()
            .names()
            .into_iter()
            .find(|name| name.name_id == POST_SCRIPT_NAME && name.is_unicode())
            .map(|name| name.to_string())
            .flatten()
            .unwrap_or_default()
    }

    #[inline]
    pub fn data(&self) -> &[u8] {
        self.face.as_slice()
    }

    #[inline]
    pub fn ascender(&self, font_size: f64) -> Pt {
        Pt(self.face.as_face_ref().ascender() as f64 * font_size)
    }

    #[inline]
    fn unit_to_pt<U: Into<f64>>(value: U, units_per_em: u16) -> Pt {
        Pt(value.into() / units_per_em as f64)
    }

    fn get_glyph_layout(face: &Face, code_point: char) -> Option<GlyphLayout<u16>> {
        let glyph_id = face.glyph_index(code_point)?;

        let width = face.glyph_hor_advance(glyph_id)?;

        Some(GlyphLayout {
            code_point,
            glyph_id,
            width,
        })
    }

    pub fn layout(&self, word: &str) -> WordLayout {
        let face = self.face.as_face_ref();
        let units_per_em = face.units_per_em();

        let mut width: u32 = 0;
        let mut glyphs = Vec::new();

        for code_point in word.chars() {
            match Self::get_glyph_layout(face, code_point) {
                Some(layout) => {
                    width += layout.width as u32;

                    let layout = GlyphLayout {
                        code_point: layout.code_point,
                        glyph_id: layout.glyph_id,
                        width: Self::unit_to_pt(layout.width, units_per_em),
                    };

                    glyphs.push(layout);
                }
                None => continue,
            }
        }

        let width = Self::unit_to_pt(width, units_per_em);

        WordLayout {
            glyphs,
            width,
        }
    }

    pub(crate) fn pdf_font<'a>(&'a self) -> PdfFont<'a> {
        PdfFont::from(self.face.as_face_ref())
    }
}

pub struct WordLayout {
    width: Pt,
    glyphs: Vec<GlyphLayout>,
}

pub struct GlyphLayout<W = Pt> {
    code_point: char,
    glyph_id: GlyphId,
    width: W,
}

impl WordLayout {
    pub fn width(&self, font_size: f64) -> Pt {
        self.width * font_size
    }

    pub fn glyphs(&self) -> &[GlyphLayout] {
        &self.glyphs
    }
}

impl GlyphLayout<Pt> {
    pub fn width(&self, font_size: f64) -> Pt {
        self.width * font_size
    }

    pub fn glyph_id(&self) -> GlyphId {
        self.glyph_id
    }
}