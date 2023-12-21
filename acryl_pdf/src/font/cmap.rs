use std::collections::BTreeMap;

use acryl_core::Vector2;

use crate::pdf::PdfObj;

use super::{glyph_info::GlyphInfo, Font};

pub struct CMap {
    // id -> (codepoint, width, height)
    map: BTreeMap<u16, (char, u16, u16)>,
    max_height: u16,
    total_width: u32,
    font_units_per_em: u16,
}

impl From<&Font> for CMap {
    fn from(value: &Font) -> Self {
        let mut map: BTreeMap<u16, (char, u16, u16)> = BTreeMap::new();

        let mut max_height: u16 = 0;
        let mut total_width: u32 = 0;

        for (_, info) in &*value.glyph_ids() {
            if let Some(GlyphInfo {
                id,
                ch,
                advance: Vector2 { x: width, .. },
                size: Vector2 { y: height, .. },
                ..
            }) = info
            {
                if *height > max_height {
                    max_height = *height;
                }

                total_width += *width as u32;

                map.insert(*id, (*ch, *width, *height));
            }
        }

        CMap {
            map,
            max_height,
            total_width,
            font_units_per_em: value.face().units_per_em(),
        }
    }
}

impl CMap {
    fn create_blocks(&self) -> Vec<Vec<(u16, char)>> {
        let mut current_first_byte: u8 = 0;

        let mut all_cmap_blocks = Vec::new();

        let mut current_cmap_block = Vec::new();

        for (glyph_id, (unicode, _, _)) in &self.map {
            if (*glyph_id >> 8) as u8 != current_first_byte || current_cmap_block.len() >= 100 {
                all_cmap_blocks.push(current_cmap_block);
                current_cmap_block = Vec::new();
                current_first_byte = (*glyph_id >> 8) as u8;
            }

            current_cmap_block.push((*glyph_id, *unicode));
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
            0,
            self.max_height as u32,
            self.total_width as u32,
            self.max_height as u32,
        ]
        .into()
    }

    pub fn create_width_vector(&self) -> PdfObj {
        let mut blocks: Vec<WidthBlock> = Vec::new();

        let font_scaling = 1000.0 / (self.font_units_per_em as f64);

        for (gid, (_, width, _)) in &self.map {
            let width = (*width as f64 * font_scaling) as i16;
            if let Some(block) = blocks.last_mut() {
                if *gid == block.next() {
                    block.widths.push(width);
                    continue;
                }
            }
            blocks.push(WidthBlock {
                start_gid: *gid,
                widths: vec![width],
            });
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
