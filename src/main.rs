use std::{fs::{File, self}, io::Write};

use acryl_pdf::{Document, Vector2};

const file_path: &'static str = "out_test.pdf";

fn main() {

    let mut doc = Document::new();

    doc.add_page(Vector2 { x: 100.into(), y: 100.into() });

    let result = doc.render().unwrap();


    fs::write(file_path, &result).unwrap();

}
