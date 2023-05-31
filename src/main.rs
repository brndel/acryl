use std::fs;

use acryl_pdf::{Document, Vector2};

const FILE_PATH: &'static str = "out/test.pdf";

fn main() {

    let mut doc = Document::new();

    doc.add_page(Vector2 { x: 100.into(), y: 100.into() });

    let result = doc.render().unwrap();

    fs::write(FILE_PATH, &result).unwrap();
}
