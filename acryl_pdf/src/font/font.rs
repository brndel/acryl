use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::fmt::Debug;
use std::{fs, io, path::Path};

use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
// use owned_ttf_parser::name_id::FULL_NAME;
use owned_ttf_parser::FaceParsingError;
use owned_ttf_parser::OwnedFace;
use owned_ttf_parser::{AsFaceRef, Face, GlyphId};

use crate::render::{Context, PdfObj, PdfObjRef};
use crate::unit::Pt;
use crate::{pdf_dict, Vector2};

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

    pub(super) fn get_glyph_info(&self, gid: u16) -> Option<GlyphInfo> {
        let face = self.face.as_face_ref();
        let glyph_id = GlyphId(gid);

        let bbox = face.glyph_bounding_box(glyph_id)?;
        let size = Vector2 {
            x: bbox.width() as u16,
            y: bbox.height() as u16,
        };

        let advance = Vector2 {
            x: face.glyph_hor_advance(glyph_id).unwrap_or(0),
            y: face.glyph_ver_advance(glyph_id).unwrap_or(0),
        };

        let gid = glyph_id.0;
        Some(GlyphInfo {
            font: &self,
            id: gid,
            advance,
            size,
        })
    }

    pub fn get_char_info(&self, c: char) -> Option<GlyphInfo> {
        let id = self.face.as_face_ref().glyph_index(c)?;
        self.used_gids.borrow_mut().insert(id.0, c);
        self.get_glyph_info(id.0)
    }

    pub fn measure_text(&self, text: &str, font_size: f64) -> Pt {
        let mut width = 0;

        for c in text.chars() {
            width += self.get_char_info(c).map_or(0, |i| i.advance.x);
        }

        self.unit_to_pt(width, font_size)
    }
}

impl Font {
    pub fn render(&self, context: &mut Context) -> PdfObjRef {
        let metrics = self.metrics();

        let file_data = self.face.as_slice().to_owned();

        let font_file = PdfObj::Stream(file_data).add_to(context);

        let cmap = CMap::from(self);
        let cid_to_unicode_map = PdfObj::Stream(cmap.to_unicode_map(&self.name)).add_to(context);

        let widths = self.create_width_vector();

        let bbox = vec![0, cmap.max_height, cmap.total_width, cmap.max_height];

        let descriptor = pdf_dict!(
            "Type" => PdfObj::Name("FontDescriptor".into()),
            "FontName" => PdfObj::Name(self.name.clone().into()),
            "Ascent" => metrics.ascender.into(),
            "Descent" => metrics.descender.into(),
            "CapHeight" => metrics.cap_height.into(),
            "ItalicAngle" => 0.into(),
            "FontFile2" => font_file.into(),
            "FontBBox" => bbox.into(),
        ).add_to(context);

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
            "DW" => PdfObj::Int(1000),
            "FontDescriptor" => descriptor.into(),
            "CIDToGIDMap" => PdfObj::Name("Identity".into())
        ).add_to(context);

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

    fn create_width_vector(&self) -> PdfObj {
        let mut blocks: Vec<WidthBlock> = Vec::new();

        let font_scaling = 1000.0 / (self.face.as_face_ref().units_per_em() as f64);

        let binding = self.glyph_ids();
        let mut ids: Vec<_> = binding.keys().into_iter().map(|id| *id).collect();
        drop(binding);

        ids.sort();

        for gid in ids {
            if let Some(GlyphInfo {
                advance: Vector2 { x: width, .. },
                ..
            }) = self.get_glyph_info(gid)
            {
                let width = (width as f64 * font_scaling) as i16;
                if let Some(block) = blocks.last_mut() {
                    if gid == block.next() {
                        block.widths.push(width);
                        continue;
                    }
                }
                blocks.push(WidthBlock {
                    start_gid: gid,
                    widths: vec![width],
                });
            }
        }

        blocks.into()
    }
}

struct WidthBlock {
    start_gid: u16,
    widths: Vec<i16>,
}

impl Into<PdfObj> for Vec<WidthBlock> {
    fn into(self) -> PdfObj {
        let mut v = Vec::new();
        for block in self {
            v.push(block.start_gid.into());
            v.push(block.widths.into());
        }

        PdfObj::Array(v)
    }
}

impl WidthBlock {
    fn next(&self) -> u16 {
        self.start_gid + self.widths.len() as u16
    }
}
