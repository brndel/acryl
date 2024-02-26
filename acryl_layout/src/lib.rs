pub mod layout_pager;
mod painter_context;
pub mod padding_values;
pub mod node;
mod layout_context;
mod dynamic_size;

use std::{fs, path::Path, time::Instant};

use rustybuzz::{shape, Face, UnicodeBuffer};

pub const FONT_DEJAVU_SERIF: &str = "/usr/share/fonts/TTF/DejaVuSerif.ttf";
pub const FONT_NOTO_SANS: &str = "/usr/share/fonts/noto/NotoSans-Regular.ttf";
pub const FONT_FREE_MONO: &str = "/usr/share/fonts/gnu-free/FreeMono.otf";

pub fn tets_layout<P: AsRef<Path>>(path: P) {
    let data = fs::read(path).unwrap();

    let start = Instant::now();
    let face = Face::from_slice(&data, 0).unwrap();
    let end = Instant::now();
    println!("parsing took {}ns", end.duration_since(start).as_nanos());

    let mut buffer = UnicodeBuffer::new();
    buffer.push_str("Hello");
    buffer.push_str("World!");
    let output = shape(&face, &[], buffer);

    let positions = output.glyph_positions();
    let infos = output.glyph_infos();

    for (position, info) in positions.iter().zip(infos) {
        let gid = info.glyph_id;
        let cluster = info.cluster;
        let x_advance = position.x_advance;
        let x_offset = position.x_offset;
        let y_offset = position.y_offset;

        // Here you would usually draw the glyphs.
        println!(
            "gid {:03?} = {:02?} @{:?},{:?}+{:?}",
            gid, cluster, x_advance, x_offset, y_offset
        );
    }
}
