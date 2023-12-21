use std::cell::{Ref, RefCell};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::{fs, io, path::Path};

use acryl_core::unit::Pt;
use acryl_core::Vector2;
use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
// use owned_ttf_parser::name_id::FULL_NAME;
use owned_ttf_parser::FaceParsingError;
use owned_ttf_parser::OwnedFace;
use owned_ttf_parser::{AsFaceRef, Face};

use crate::pdf::{PdfObj, PdfObjRef};
use crate::pdf_dict;
use crate::writer::PdfWriter;

use super::cmap::CMap;
use super::font_metrics::FontMetrics;
use super::glyph_info::GlyphInfo;

#[derive(Debug)]
pub enum FontLoadError {
    File(io::Error),
    Parse(FaceParsingError),
}

#[derive(Debug)]
pub struct Font {
    face: OwnedFace,
    name: String,
    pub(super) units_per_em: u16,
    glyph_info_cache: RefCell<BTreeMap<char, Option<GlyphInfo>>>,
    // word_width_cache: RefCell<HashMap<String, Pt>>,
}

impl Font {
    pub const DEFAULT_GLYPH_UNITS: u16 = 1000;

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, FontLoadError> {
        let font_data = fs::read(path).map_err(|e| FontLoadError::File(e))?;

        let face = match OwnedFace::from_vec(font_data, 0) {
            Ok(face) => face,
            Err(err) => return Err(FontLoadError::Parse(err)),
        };

        let name = face
            .as_face_ref()
            .names()
            .into_iter()
            .find(|name| name.name_id == POST_SCRIPT_NAME && name.is_unicode())
            .and_then(|name| name.to_string())
            .unwrap_or_default();

        let units_per_em = face.as_face_ref().units_per_em();

        Ok(Self {
            face,
            name,
            units_per_em,
            glyph_info_cache: Default::default(),
            // word_width_cache: Default::default(),
        })
    }

    #[inline]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[inline]
    pub fn metrics(&self) -> FontMetrics {
        FontMetrics::from(self)
    }

    #[inline]
    pub(super) fn face(&self) -> &Face {
        self.face.as_face_ref()
    }

    #[inline]
    pub(super) fn unit_to_pt<U: Into<f64>>(units_per_em: u16, value: U) -> Pt {
        Pt(value.into() / units_per_em as f64)
    }

    pub fn get_glyph_info(&self, ch: char) -> Option<GlyphInfo> {
        if let Some(info) = self.glyph_info_cache.borrow().get(&ch) {
            return info.to_owned();
        }

        let face = self.face();
        let glyph_id = match face.glyph_index(ch) {
            Some(id) => id,
            None => {
                self.glyph_info_cache.borrow_mut().insert(ch, None);
                return None;
            }
        };

        let bbox = face
            .glyph_bounding_box(glyph_id)
            .map_or((0, 0), |bbox| (bbox.width(), bbox.height()));
        let size = Vector2 {
            x: bbox.0 as u16,
            y: bbox.1 as u16,
        };

        let advance = Vector2 {
            x: face.glyph_hor_advance(glyph_id).unwrap_or(0),
            y: face.glyph_ver_advance(glyph_id).unwrap_or(0),
        };

        let id = glyph_id.0;
        let info = GlyphInfo {
            id,
            ch,
            advance,
            size,
            units_per_em: self.units_per_em,
        };

        self.glyph_info_cache.borrow_mut().insert(ch, Some(info));

        self.glyph_info_cache.borrow().get(&ch).unwrap().to_owned()
    }

    pub(crate) fn get_char_id(&self, c: char) -> Option<u16> {
        self.face().glyph_index(c).map(|id| id.0)
    }

    pub fn measure_text(&self, text: &str, font_size: f64) -> Pt {
        // if let Some(width) = self.word_width_cache.borrow().get(text) {
        //     return *width * font_size;
        // }

        let mut width: Pt = Pt::default();

        for ch in text.chars() {
            width += self
                .get_glyph_info(ch)
                .map_or_else(|| self.default_glyph_width(), |i| i.advance(1.0).x);
        }

        // self.word_width_cache.borrow_mut().insert(text.to_owned(), width);

        width * font_size
    }

    #[inline]
    pub(crate) fn default_glyph_width(&self) -> Pt {
        Self::unit_to_pt(self.units_per_em, Self::DEFAULT_GLYPH_UNITS)
    }
}

impl Font {
    pub fn render<T: PdfWriter>(&self, writer: &mut T) -> PdfObjRef {
        let metrics = self.metrics();

        let file_data = self.face.as_slice().to_owned();

        let font_file = PdfObj::Stream(file_data.into()).add_to(writer);

        let cmap = CMap::from(self);
        let cid_to_unicode_map =
            PdfObj::Stream(cmap.create_to_unicode_map(&self.name).into()).add_to(writer);

        let widths = cmap.create_width_vector();

        let bbox = cmap.create_bbox();

        let descriptor = pdf_dict!(
            "Type" => PdfObj::Name("FontDescriptor".into()),
            "FontName" => PdfObj::Name(self.name.clone().into()),
            "Ascent" => metrics.ascender.into(),
            "Descent" => metrics.descender.into(),
            "Leading" => metrics.leading.into(),
            "CapHeight" => metrics.cap_height.into(),
            "ItalicAngle" => 0.into(),
            "FontFile2" => font_file.into(),
            "FontBBox" => bbox,
        )
        .add_to(writer);

        let desc_font = pdf_dict!(
            "Type" => PdfObj::Name("Font".into()),
            "Subtype" => PdfObj::Name("CIDFontType2".into()),
            "BaseFont" => PdfObj::Name(self.name.clone().into()),
            "CIDSystemInfo" => pdf_dict!(
                "Registry" => PdfObj::StringLiteral("Adobe".into()),
                "Ordering" => PdfObj::StringLiteral("Identity".into()),
                "Supplement" => PdfObj::Int(0),
            ),
            "W" => widths,
            "DW" => Self::DEFAULT_GLYPH_UNITS.into(),
            "FontDescriptor" => descriptor.into(),
            "CIDToGIDMap" => PdfObj::Name("Identity".into())
        )
        .add_to(writer);

        let font_dict = pdf_dict!(
            "Type" => PdfObj::Name("Font".into()),
            "Subtype" =>  PdfObj::Name("Type0".into()),
            "BaseFont" => PdfObj::Name(self.name.clone().into()),
            "Encoding" => PdfObj::Name("Identity-H".into()),
            "ToUnicode" => cid_to_unicode_map.into(),
            "DescendantFonts" => vec![desc_font].into(),
        );

        writer.add(font_dict)
    }

    pub(super) fn glyph_ids(&self) -> Ref<BTreeMap<char, Option<GlyphInfo>>> {
        self.glyph_info_cache.borrow()
    }
}
