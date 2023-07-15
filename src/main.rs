use std::{fs::File, rc::Rc, time::Instant};

use acryl_pdf::{
    font::{ExternalFont, FontRef},
    stream::{Color, Streambuilder},
    unit::Pt,
    util::Area,
    Document, Vector2,
};

const FILE_PATH: &'static str = "out/test.pdf";

fn main() {
    let start = Instant::now();

    let file = File::create(FILE_PATH).unwrap();

    build_file(file);

    let end = Instant::now();

    let dur = end.duration_since(start);
    println!("Wrote file '{}' [{}ms]", FILE_PATH, dur.as_millis());
}

fn build_file(mut file: File) {
    let mut doc = Document::new();

    let font_dejavu = Rc::new(
        ExternalFont::load("/usr/share/fonts/TTF/DejaVuSerif.ttf")
            .expect("font could not be loaded"),
    );
    let font_notosans = Rc::new(
        ExternalFont::load("/usr/share/fonts/noto/NotoSans-Regular.ttf")
            .expect("font could not be loaded"),
    );
    let font_freemono = Rc::new(
        ExternalFont::load("/usr/share/fonts/gnu-free/FreeMono.otf")
            .expect("font could not be loaded"),
    );

    let font_dejavu_ref = doc.add_font(font_dejavu.clone());
    let font_notosans_ref = doc.add_font(font_notosans.clone());
    let font_freemono_ref = doc.add_font(font_freemono.clone());

    let page = doc.add_page(None);

    let mut builder = Streambuilder::new(page);

    let text = "the_quick-brown/fox_jumps'over*the#lazy.dogäöü€@<>()";

    draw_marked_text(
        &text,
        Vector2 {
            x: Pt(10.0),
            y: Pt(20.0),
        },
        &mut builder,
        font_dejavu,
        font_dejavu_ref,
        12.0,
    );
    draw_marked_text(
        &text,
        Vector2 {
            x: Pt(10.0),
            y: Pt(40.0),
        },
        &mut builder,
        font_notosans,
        font_notosans_ref,
        12.0,
    );
    draw_marked_text(
        &text,
        Vector2 {
            x: Pt(10.0),
            y: Pt(60.0),
        },
        &mut builder,
        font_freemono,
        font_freemono_ref,
        12.0,
    );


    builder.render();

    doc.render(&mut file).unwrap()
}

fn draw_marked_text<S: AsRef<str>>(
    text: S,
    position: Vector2<Pt>,
    builder: &mut Streambuilder,
    font: Rc<ExternalFont>,
    font_ref: FontRef,
    font_size: f64,
) {
    let width = font.measure_text(text.as_ref(), font_size);

    let metrics = font.metrics().sized(font_size);
    println!("[{}] asc: {} desc: {} height: {}", font.name(), metrics.ascender(), metrics.descender(), metrics.height());

    let rect_position = position.clone() - Vector2 { x: Pt(0.0), y: metrics.ascender() };

    builder.draw_rect(
        Area {
            position: rect_position,
            size: Vector2 {
                x: width,
                y: metrics.height(),
            },
        },
        Color::RGB(1.0, 0.8, 0.6),
    );

    let mut text_builder = builder.text(font, font_ref, font_size);
    text_builder.set_scale(100);
    text_builder.set_position(position);
    text_builder.draw_text(text.as_ref());

    drop(text_builder);
}
