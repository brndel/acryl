use std::cell::RefCell;
use std::collections::BTreeSet;
use std::fmt::Debug;
use std::{fs, io, path::Path};

use acryl_core::math::Pt;
use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
use owned_ttf_parser::{AsFaceRef, FaceParsingError, GlyphId, OwnedFace};

use super::pdf_font::PdfFont;

/// �
const INVALID_CHAR_FALLBACK: char = '\u{FFFD}';

#[derive(Debug)]
pub enum FontLoadError {
    File(io::Error),
    Parse(FaceParsingError),
}

pub struct Font {
    name: String,
    face: OwnedFace,
    used_chars: RefCell<BTreeSet<char>>,
}

impl Debug for Font {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Font").field("face", &"[...]").finish()
    }
}

impl Font {
    pub const DEFAULT_GLYPH_UNITS: u16 = 1000;

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, FontLoadError> {
        let path = path.as_ref();
        let data = fs::read(&path).map_err(|e| FontLoadError::File(e))?;

        Self::from_data(path.display().to_string(), data, 0)
    }

    pub fn from_data(name: String, data: Vec<u8>, index: u32) -> Result<Self, FontLoadError> {
        let face = OwnedFace::from_vec(data, index).map_err(FontLoadError::Parse)?;

        Ok(Self {
            name,
            face,
            used_chars: Default::default(),
        })
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
        let face = self.face.as_face_ref();
        Self::unit_to_pt(face.ascender(), face.units_per_em()) * font_size
    }

    #[inline]
    fn unit_to_pt<U: Into<f64>>(value: U, units_per_em: u16) -> Pt {
        Pt(value.into() / units_per_em as f64)
    }

    fn get_glyph_layout(&self, code_point: char) -> Option<GlyphLayout<u16>> {
        self.used_chars.borrow_mut().insert(code_point);

        let face = self.face.as_face_ref();
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
            let layout = match self.get_glyph_layout(code_point) {
                Some(layout) => layout,
                None => {
                    println!("font {:?} does not provide '{}' glyph", self.name, code_point);

                    match self.get_glyph_layout(INVALID_CHAR_FALLBACK) {
                        Some(layout) => layout,
                        None => continue
                    }
                },
            };
            width += layout.width as u32;

            let layout = GlyphLayout {
                code_point: layout.code_point,
                glyph_id: layout.glyph_id,
                width: Self::unit_to_pt(layout.width, units_per_em),
            };

            glyphs.push(layout);
        }

        let width = Self::unit_to_pt(width, units_per_em);

        WordLayout { glyphs, width }
    }

    pub(crate) fn pdf_font<'a>(&'a self) -> PdfFont<'a> {
        PdfFont::new(self.face.as_face_ref(), self.used_chars.borrow().iter())
    }
}

pub struct WordLayout {
    width: Pt,
    glyphs: Vec<GlyphLayout>,
}

#[derive(Debug, Clone)]
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
