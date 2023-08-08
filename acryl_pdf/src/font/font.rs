use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, io, path::Path};

use acryl_core::Vector2;
use acryl_core::unit::Pt;
use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
// use owned_ttf_parser::name_id::FULL_NAME;
use owned_ttf_parser::FaceParsingError;
use owned_ttf_parser::OwnedFace;
use owned_ttf_parser::{AsFaceRef, Face, GlyphId};

use crate::render::{Context, PdfObj, PdfObjRef};
use crate::pdf_dict;

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
    units_per_em: u16,
    used_gids: RefCell<HashMap<u16, char>>,
}

impl Font {
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
            used_gids: RefCell::default(),
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
    pub(super) fn unit_to_pt<U: Into<f64>, S: Into<f64>>(&self, value: U, font_size: S) -> Pt {
        Pt(value.into() / (self.units_per_em as f64) * font_size.into())
    }

    pub(super) fn get_glyph_info(&self, gid: u16) -> GlyphInfo {
        let face = self.face();
        let glyph_id = GlyphId(gid);

        let bbox = face.glyph_bounding_box(glyph_id).map_or((0, 0), |bbox| (bbox.width(), bbox.height()));
        let size = Vector2 {
            x: bbox.0 as u16,
            y: bbox.1 as u16,
        };

        let advance = Vector2 {
            x: face.glyph_hor_advance(glyph_id).unwrap_or(0),
            y: face.glyph_ver_advance(glyph_id).unwrap_or(0),
        };

        let gid = glyph_id.0;
        GlyphInfo {
            font: &self,
            id: gid,
            advance,
            size,
        }
    }

    pub(crate) fn get_char_id(&self, c: char) -> Option<u16> {
        self.face().glyph_index(c).map(|id| id.0)
    }

    pub fn get_char_info(&self, c: char) -> Option<GlyphInfo> {
        let id = self.face().glyph_index(c)?.0;
        self.used_gids.borrow_mut().insert(id, c);
        Some(self.get_glyph_info(id))
    }

    pub fn measure_text(&self, text: &str, font_size: f64) -> Pt {
        let mut width: u32 = 0;

        for c in text.chars() {
            width += self
                .get_char_info(c)
                .map_or(Self::default_glyph_width(), |i| i.advance.x) as u32;
        }

        self.unit_to_pt(width, font_size)
    }

    #[inline]
    pub(crate) const fn default_glyph_width() -> u16 {
        1000
    }
}

impl Font {
    pub fn render(&self, context: &mut Context) -> PdfObjRef {
        let metrics = self.metrics();

        let file_data = self.face.as_slice().to_owned();

        let font_file = PdfObj::Stream(file_data).add_to(context);

        let cmap = CMap::from(self);
        let cid_to_unicode_map = PdfObj::Stream(cmap.create_to_unicode_map(&self.name)).add_to(context);

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
        .add_to(context);

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
            "DW" => Self::default_glyph_width().into(),
            "FontDescriptor" => descriptor.into(),
            "CIDToGIDMap" => PdfObj::Name("Identity".into())
        )
        .add_to(context);

        let font_dict = pdf_dict!(
            "Type" => PdfObj::Name("Font".into()),
            "Subtype" =>  PdfObj::Name("Type0".into()),
            "BaseFont" => PdfObj::Name(self.name.clone().into()),
            "Encoding" => PdfObj::Name("Identity-H".into()),
            "ToUnicode" => cid_to_unicode_map.into(),
            "DescendantFonts" => vec![desc_font].into(),
        );

        context.add(font_dict)
    }

    pub(super) fn glyph_ids(&self) -> Ref<HashMap<u16, char>> {
        self.used_gids.borrow()
    }
}
