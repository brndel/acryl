mod layout;
mod util;
mod doc_config;

use std::{
    fs::{self, File},
    time::Instant,
};


use acryl_parser::{
    ast::ContentToken,
    file::DocFile,
    parse, ParsedFile,
};
use acryl_pdf::{
    font::Font, resource_manager::ResourceManager, stream::Streambuilder, structure::{Document, Page}, write::PdfDocument
};

use layout::{linear_layout::LinearLayout, text::TextElement};


use crate::{layout::LayoutElement, doc_config::DocumentConfig};

const SAMPLE_FILE_PATH: &str = "examples/minimal.acryl";
const OUT_FILE_PATH: &str = "out/minimal.pdf";

const FONT_DEJAVU_SERIF: &str = "/usr/share/fonts/TTF/DejaVuSerif.ttf";
const FONT_NOTO_SANS: &str = "/usr/share/fonts/noto/NotoSans-Regular.ttf";
const FONT_FREE_MONO: &str = "/usr/share/fonts/gnu-free/FreeMono.otf";

fn main() {
    let start = Instant::now();

    let source = fs::read_to_string(SAMPLE_FILE_PATH).expect("could not open sample acryl file");

    let doc = parse_file(&source).unwrap();
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

fn parse_file(source: &str) -> Option<DocFile> {
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
    let config: DocumentConfig = doc.header().try_into().map_err(|err| panic!("could not parse header '{}'", err)).unwrap();

    println!("{:?}", config);

    let mut resource_manager = ResourceManager::new();
    let default_font =
        resource_manager.add_font(Font::load(FONT_DEJAVU_SERIF).expect("Font file not found"));

        
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
    
    let mut page = Page::new(config.default_page_size.clone());

    let mut builder = Streambuilder::new(&mut page);
    layout.render(builder.get_area().clone(), &mut builder);
    builder.render();

    let document = Document::new(config.info, resource_manager, vec![page]);
    let mut out_file = File::create(OUT_FILE_PATH).expect("could not create out file");

    let document = PdfDocument::new(document);

    document
        .write(&mut out_file)
        .expect("error while writing document");

    Some(out_file)
}
