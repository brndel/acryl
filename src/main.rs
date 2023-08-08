mod layout;
mod util;

use std::{
    fs::{self, File},
    str::FromStr,
    time::Instant,
};

use acryl_parser::{
    ast::{CodeToken, ContentToken},
    file::{DocFile, DocFileHeader},
    parse, ParsedFile,
};
use acryl_pdf::{font::Font, stream::Streambuilder, Document, DocumentConfig};

use layout::{linear_layout::LinearLayout, text::TextElement};
use util::page_size::PageSize;

use crate::layout::LayoutElement;

const SAMPLE_FILE_PATH: &str = "examples/minimal.acryl";
const OUT_FILE_PATH: &str = "out/minimal.pdf";

const FONT_DEJAVU_SERIF: &str = "/usr/share/fonts/TTF/DejaVuSerif.ttf";
const FONT_NOTO_SANS: &str = "/usr/share/fonts/noto/NotoSans-Regular.ttf";
const FONT_FREE_MONO: &str = "/usr/share/fonts/gnu-free/FreeMono.otf";

fn main() {
    let start = Instant::now();

    let source = fs::read_to_string(SAMPLE_FILE_PATH).expect("could not open sample acryl file");

    let doc = parse_sample(&source).unwrap();
    let file = match build_pdf_from_doc(doc) {
        Some(file) => file,
        None => return,
    };

    let size = file
        .metadata()
        .expect("could not open metadata of file")
        .len();
    let size_kb = size / 1000;

    let end = Instant::now();

    let dur = end.duration_since(start);

    println!(
        "Wrote file '{}' [{}ms, {}.{}mb]",
        OUT_FILE_PATH,
        dur.as_millis(),
        size_kb / 1000,
        size_kb % 1000
    );
}

fn parse_sample(source: &str) -> Option<DocFile> {
    let result = parse(&source);

    if let Some(ParsedFile::Doc(doc)) = result {
        Some(doc)
    } else {
        println!("{:#?}", result);
        println!("invalid result type");
        None
    }
}

fn build_pdf_from_doc(doc: DocFile) -> Option<File> {
    let config = match parse_document_config(doc.header()) {
        Ok(config) => config,
        Err(err) => {
            println!("{}", err);
            return None;
        }
    };

    println!("{:?}", config);

    let mut document = Document::new(config);
    let default_font =
        document.add_font(Font::load(FONT_DEJAVU_SERIF).expect("Font file not found"));

    let mut layout = LinearLayout::vertical();

    let mut text = TextElement::new(&default_font, 12.0);

    for token in doc.content().tokens() {
        match token {
            ContentToken::Word(word) => text.add_word(word),
            ContentToken::Fn {
                name,
                key,
                arguments,
                content,
            } => todo!(),
        }
    }
    layout.add(text);

    let mut out_file = File::create(OUT_FILE_PATH).expect("could not create out file");
    let page = document.add_page(None);
    let mut builder = Streambuilder::new(page);

    layout.render(builder.get_area().clone(), &mut builder);

    builder.render();

    document
        .render(&mut out_file)
        .expect("error while writing document");

    Some(out_file)
}

fn parse_document_config(header: &DocFileHeader) -> Result<DocumentConfig, &'static str> {
    let author = match header.get("author") {
        Some(token) => Some(
            token
                .as_str()
                .ok_or("author needs to be of type str")?
                .to_owned(),
        ),
        None => None,
    };

    let page_size = match header.get("pageSize") {
        Some(size) => size
            .as_ident()
            .ok_or("'pageSize' needs to be of type ident")?
            .parse()
            .map_err(|_| "Unknow Page Size")?,
        None => PageSize::default(),
    };

    Ok(DocumentConfig {
        author,
        default_page_size: page_size.get_size(),
    })
}
