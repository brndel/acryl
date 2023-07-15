use std::collections::{BTreeMap, HashMap};
use std::{fs, io, path::Path};

use owned_ttf_parser::cmap::Subtable2;
use owned_ttf_parser::name_id::POST_SCRIPT_NAME;
// use owned_ttf_parser::name_id::FULL_NAME;
use owned_ttf_parser::FaceParsingError;
use owned_ttf_parser::OwnedFace;
use owned_ttf_parser::{AsFaceRef, Face, GlyphId};

use crate::render::{Context, PdfObj, PdfObjRef};
use crate::unit::Pt;
use crate::{pdf_dict, Vector2};

use super::face_metrics::FaceMetrics;
use super::glyph_info::GlyphInfo;

#[derive(Debug)]
pub enum FontLoadError {
    File(io::Error),
    Parse(FaceParsingError),
}

pub struct ExternalFont {
    face: OwnedFace,
    name: String,
    metrics: FaceMetrics,
}

struct GlyphMetrics {
    width: u32,
    height: u32,
}

impl ExternalFont {
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

        let metrics = FaceMetrics::from(face.as_face_ref());

        Ok(Self {
            face,
            name,
            metrics,
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn metrics(&self) -> &FaceMetrics {
        &self.metrics
    }

    fn get_glyph_metrics(&self, id: u16) -> Option<GlyphMetrics> {
        let face = self.face.as_face_ref();
        let id = GlyphId(id);
        let width = face.glyph_hor_advance(id)? as u32;
        let height = face.glyph_bounding_box(id)?.height() as u32;
        Some(GlyphMetrics { width, height })
    }

    pub(crate) fn unit_to_pt<U: Into<f64>>(&self, unit: U, font_size: f64) -> Pt {
        Pt(unit.into() / (self.face.as_face_ref().units_per_em() as f64) * font_size)
    }

    fn get_glyph_info(&self, glyph_id: GlyphId) -> Option<GlyphInfo> {
        let face = self.face.as_face_ref();

        let name = face.glyph_name(glyph_id)?;
        let advance = Vector2 {
            x: face.glyph_hor_advance(glyph_id).unwrap_or(0),
            y: face.glyph_ver_advance(glyph_id).unwrap_or(0),
        };

        let id = glyph_id.0;
        Some(GlyphInfo::new(&self, id, name, advance))
    }

    pub fn get_char_info(&self, c: char) -> Option<GlyphInfo> {
        self.get_glyph_info(self.face.as_face_ref().glyph_index(c)?)
    }

    pub fn measure_text(&self, text: &str, font_size: f64) -> Pt {
        let mut width = 0;

        fn get_width(face: &Face, c: char) -> Option<u16> {
            let glyph_id = face.as_face_ref().glyph_index(c)?;
            face.as_face_ref().glyph_hor_advance(glyph_id)
        }

        for c in text.chars() {
            width += get_width(self.face.as_face_ref(), c).unwrap_or(0);
        }

        self.unit_to_pt(width, font_size)
    }
}

impl ExternalFont {
    pub fn render(&self, context: &mut Context) -> PdfObjRef {
        let metrics = self.metrics();

        let file_data = self.face.as_slice().to_owned();

        let font_file = PdfObj::Stream(file_data);
        let font_file = context.add(font_file);

        let cmap = CMap::from(self);
        let cid_to_unicode_map = cmap.to_unicode_map(&self.name);
        let cid_to_unicode_map = context.add(PdfObj::Stream(cid_to_unicode_map));

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
        );
        let descriptor = context.add(descriptor);

        let widths = self.create_width_vector();

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
        );

        let desc_font = context.add(desc_font);

        let font_dict = pdf_dict!(
            "Type" => PdfObj::Name("Font".into()),
            "Subtype" =>  PdfObj::Name("Type0".into()),
            "BaseFont" => PdfObj::Name(self.name.clone().into()),
            // "BaseFont" => PdfObj::Name("Hey".into()),
            "Encoding" => PdfObj::Name("Identity-H".into()),
            "ToUnicode" => cid_to_unicode_map.into(),
            "DescendantFonts" => vec![desc_font].into(),
        );

        context.add(font_dict)
    }

    fn glyph_ids(&self) -> HashMap<u16, char> {
        let subtable = Subtable2::parse(self.face.as_slice()).unwrap();
        let face = self.face.as_face_ref();

        let mut map = HashMap::with_capacity(face.number_of_glyphs().into());
        subtable.codepoints(|cp| {
            if let Ok(ch) = char::try_from(cp) {
                if let Some(idx) = subtable.glyph_index(cp).filter(|idx| idx.0 > 0) {
                    map.entry(idx.0).or_insert(ch);
                }
            }
        });
        map
    }

    fn create_width_vector(&self) -> PdfObj {
        struct Block {
            start_gid: u16,
            widths: Vec<i64>,
        }

        impl Into<PdfObj> for Vec<Block> {
            fn into(self) -> PdfObj {
                let mut v = Vec::new();
                for block in self {
                    v.push(block.start_gid.into());
                    v.push(block.widths.into());
                }

                PdfObj::Array(v)
            }
        }

        impl Block {
            fn next(&self) -> u16 {
                self.start_gid + self.widths.len() as u16
            }
        }

        let mut blocks: Vec<Block> = Vec::new();

        let font_scaling = 1000.0 / (self.face.as_face_ref().units_per_em() as f64);

        for gid in 0..self.face.as_face_ref().number_of_glyphs() {
            if let Some(GlyphMetrics { width, .. }) = self.get_glyph_metrics(gid) {
                let width = (width as f64 * font_scaling) as i64;
                if let Some(block) = blocks.last_mut() {
                    if gid == block.next() {
                        block.widths.push(width);
                        continue;
                    }
                }
                blocks.push(Block {
                    start_gid: gid,
                    widths: vec![width],
                });
            }
        }

        blocks.into()
    }
}

struct CMap {
    // id -> (codepoint, width, height)
    map: BTreeMap<u32, (u32, u32, u32)>,
    max_height: u32,
    total_width: u32,
}

impl From<&ExternalFont> for CMap {
    fn from(value: &ExternalFont) -> Self {
        let mut map: BTreeMap<u32, (u32, u32, u32)> = BTreeMap::new();
        map.insert(0, (0, 1000, 1000));

        let mut max_height = 0;
        let mut total_width = 0;

        for (glyph_id, ch) in value.glyph_ids() {
            if let Some(metrics) = value.get_glyph_metrics(glyph_id) {
                if metrics.height > max_height {
                    max_height = metrics.height;
                }

                total_width += metrics.width;
                map.insert(glyph_id as u32, (ch as u32, metrics.width, metrics.height));
            }
        }

        CMap {
            map,
            max_height,
            total_width,
        }
    }
}

impl CMap {
    fn create_blocks(&self) -> Vec<Vec<(u32, u32)>> {
        let mut widths = Vec::new();

        let mut current_first_bit: u16 = 0;

        let mut all_cmap_blocks = Vec::new();

        let mut current_cmap_block = Vec::new();

        for (glyph_id, (unicode, width, _)) in &self.map {
            if (*glyph_id >> 8) as u16 != current_first_bit || current_cmap_block.len() >= 100 {
                all_cmap_blocks.push(current_cmap_block);
                current_cmap_block = Vec::new();
                current_first_bit = (*glyph_id >> 8) as u16;
            }

            current_cmap_block.push((*glyph_id, *unicode));
            widths.push((*glyph_id, *width));
        }

        all_cmap_blocks.push(current_cmap_block);

        all_cmap_blocks
    }

    fn to_unicode_map(&self, name: &str) -> Vec<u8> {
        let mut map = format!(include_str!("../../assets/gid_to_unicode_beg.txt"), name);

        map.push_str("\r\n");

        let cmap_blocks = self.create_blocks();

        for block in cmap_blocks
            .into_iter()
            .filter(|block| !block.is_empty() && block.len() < 100)
        {
            map.push_str(&format!("{} beginbfchar\r\n", block.len()));
            for (glyph_id, unicode) in block {
                map.push_str(&format!("<{:04x}> <{:04x}>\r\n", glyph_id, unicode));
            }
            map.push_str("endbfchar\r\n");
        }

        map.push_str(include_str!("../../assets/gid_to_unicode_end.txt"));

        map.push_str("\r\n");

        map.into_bytes()
    }
}
