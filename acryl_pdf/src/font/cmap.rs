use std::collections::BTreeMap;

use owned_ttf_parser::{Face, GlyphId, Rect};

use crate::data::PdfObj;

pub struct CMap {
    // id -> (codepoint, width, height)
    map: BTreeMap<GlyphId, GlyphData>,
    bounding_box: Rect,
    font_units_per_em: u16,
}

struct GlyphData {
    code_point: u32,
    width: u16,
}

impl CMap {
    pub fn new<'a, 'b>(face: &'a Face, used_chars: impl Iterator<Item = &'b char>) -> Self {
        let mut map: BTreeMap<GlyphId, GlyphData> = BTreeMap::new();

        for code_point in used_chars {
            if let Some(glyph_id) = face.glyph_index(*code_point) {
                map.entry(glyph_id).or_insert_with(|| {
                    let width = face.glyph_hor_advance(glyph_id).unwrap_or_default();

                    GlyphData {
                        code_point: *code_point as u32,
                        width,
                    }
                });
            }
        }


        CMap {
            map,
            bounding_box: 
            face.global_bounding_box(),
            font_units_per_em: face.units_per_em(),
        }
    }
}

impl CMap {
    fn create_blocks(&self) -> Vec<Vec<(u16, u32)>> {
        let mut current_first_byte: u8 = 0;

        let mut all_cmap_blocks = Vec::new();

        let mut current_cmap_block = Vec::new();

        for (GlyphId(gid), GlyphData { code_point, .. }) in &self.map {
            if (*gid >> 8) as u8 != current_first_byte || current_cmap_block.len() >= 100 {
                all_cmap_blocks.push(current_cmap_block);
                current_cmap_block = Vec::new();
                current_first_byte = (*gid >> 8) as u8;
            }

            current_cmap_block.push((*gid, *code_point));
        }

        all_cmap_blocks.push(current_cmap_block);

        all_cmap_blocks
    }

    pub fn create_to_unicode_map(&self, name: &str) -> Vec<u8> {
        let mut map = format!(include_str!("../../assets/gid_to_unicode_beg.txt"), name);

        map.push_str("\r\n");

        let cmap_blocks = self.create_blocks();

        for block in cmap_blocks
            .into_iter()
            .filter(|block| !block.is_empty() && block.len() < 100)
        {
            map.push_str(&format!("{} beginbfchar\r\n", block.len()));
            for (glyph_id, unicode) in block {
                map.push_str(&format!("<{:04x}> <{:04x}>\r\n", glyph_id, unicode as u16));
            }
            map.push_str("endbfchar\r\n");
        }

        map.push_str(include_str!("../../assets/gid_to_unicode_end.txt"));

        map.push_str("\r\n");

        map.into_bytes()
    }

    pub fn create_bbox(&self) -> PdfObj {
        vec![
            self.bounding_box.x_min as u32,
            self.bounding_box.y_min as u32,
            self.bounding_box.x_max as u32,
            self.bounding_box.y_max as u32,
        ]
        .into()
    }

    pub fn create_width_vector(&self) -> PdfObj {
        let mut blocks: Vec<WidthBlock> = Vec::new();

        let font_scaling = 1000 / self.font_units_per_em;

        for (gid, GlyphData { width, .. }) in &self.map {
            let width = width * font_scaling;
            if let Some(block) = blocks.last_mut() {
                if gid.0 == block.next() {
                    block.widths.push(width);
                    continue;
                }
            }
            blocks.push(WidthBlock {
                start_gid: gid.0,
                widths: vec![width],
            });
        }

        blocks.into()
    }
}

struct WidthBlock {
    start_gid: u16,
    widths: Vec<u16>,
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
