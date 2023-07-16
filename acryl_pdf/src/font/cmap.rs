use std::collections::BTreeMap;

use crate::Vector2;

use super::{Font, glyph_info::GlyphInfo};


pub struct CMap {
    // id -> (codepoint, width, height)
    map: BTreeMap<u32, (u32, u32, u32)>,
    pub max_height: u32,
    pub total_width: u32,
}

impl From<&Font> for CMap {
    fn from(value: &Font) -> Self {
        let mut map: BTreeMap<u32, (u32, u32, u32)> = BTreeMap::new();
        map.insert(0, (0, 1000, 1000));

        let mut max_height: u32 = 0;
        let mut total_width: u32 = 0;

        for (gid, ch) in &*value.glyph_ids() {
            if let Some(GlyphInfo {
                advance: Vector2 { x: width, .. },
                size: Vector2 { y: height, .. },
                ..
            }) = value.get_glyph_info(*gid)
            {
                let height = height as u32;
                let width = width as u32;
                if height > max_height {
                    max_height = height;
                }

                total_width += width;
                map.insert(*gid as u32, (*ch as u32, width, height));
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
    pub fn create_blocks(&self) -> Vec<Vec<(u32, u32)>> {
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

    pub fn to_unicode_map(&self, name: &str) -> Vec<u8> {
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
