use std::fs;

use acryl_pdf::{unit::Pt, Document, Vector2};

const FILE_PATH: &'static str = "out/test.pdf";

fn main() {
    let mut doc = Document::new();

    doc.add_page(Vector2 {
        x: Pt(100.0),
        y: Pt(100.0),
    });


    let result = doc.render().unwrap();

    fs::write(FILE_PATH, &result).unwrap();
}
