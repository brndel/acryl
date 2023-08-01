mod layout;
mod util;

use std::{fs::File, time::Instant};

use acryl_pdf::{
    font::{Font, FontRef},
    stream::{Color, Streambuilder},
    unit::Pt,
    Document,
};

use layout::{
    color_box::ColorBox,
    linear_layout::{CrossAxisAlignment, LinearLayout, MainAxisAlignment},
    padding::Padding,
    size_box::SizeBox,
    text::TextElement,
    LayoutElement,
};

const LOREM_IPSUM_100: &'static str = "Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet. Lorem ipsum dolor sit amet, consetetur sadipscing elitr, sed diam nonumy eirmod tempor invidunt ut labore et dolore magna aliquyam erat, sed diam voluptua. At vero eos et accusam et justo duo dolores et ea rebum. Stet clita kasd gubergren, no sea takimata sanctus est Lorem ipsum dolor sit amet.";
const FILE_PATH: &'static str = "out/test.pdf";

fn main() {
    let start = Instant::now();

    let mut file = File::create(FILE_PATH).unwrap();

    build_file(&mut file);
    
    let size = file
        .metadata()
        .expect("could not open metadata of file")
        .len();
    let size_kb = size / 1000;

    let end = Instant::now();

    let dur = end.duration_since(start);
    println!(
        "Wrote file '{}' [{}ms, {}.{}mb]",
        FILE_PATH,
        dur.as_millis(),
        size_kb / 1000,
        size_kb % 1000
    );
}

fn build_file(mut file: &mut File) {
    let mut doc = Document::new();

    let font_dejavu =
        Font::load("/usr/share/fonts/TTF/DejaVuSerif.ttf").expect("font could not be loaded");
    // let font_notosans =
    //     Font::load("/usr/share/fonts/noto/NotoSans-Regular.ttf").expect("font could not be loaded");
    // let font_freemono =
    //     Font::load("/usr/share/fonts/gnu-free/FreeMono.otf").expect("font could not be loaded");

    let default_font = doc.add_font(font_dejavu);

    let page = doc.add_page(None);

    let mut builder = Streambuilder::new(page);

    draw_layout(&mut builder, &default_font);

    builder.render();

    doc.render(&mut file).unwrap()
}

fn draw_layout(builder: &mut Streambuilder, font: &FontRef) {
    let mut layout = LinearLayout::vertical()
        .main_axis(MainAxisAlignment::Start)
        .cross_axis(CrossAxisAlignment::Start)
        .spacing(Pt(8.0));

    let text_element = TextElement::new("Hello Layout World!".into(), font, 24.0);
    layout.add(ColorBox::new(text_element, Color::rgb_from_hex(0xFD004C)));

    let text_element = TextElement::new(LOREM_IPSUM_100.into(), font, 12.0);
    layout.add(ColorBox::new(text_element, Color::rgb_from_hex(0xE800FD)));

    layout.add(ColorBox::new(
        SizeBox::new(80.0, 80.0),
        Color::rgb_from_hex(0x25AC37),
    ));

    layout.add(ColorBox::new(
        SizeBox::new(70.0, 150.0),
        Color::rgb_from_hex(0x1446EE),
    ));

    let layout = Padding::new(layout, Pt(8.0));

    layout.render(builder.get_area().clone(), builder);
}
