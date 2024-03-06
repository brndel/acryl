mod doc_config;
mod util;

use std::{
    fs::{self, File},
    time::Instant,
};

use acryl_core::{
    math::{Pt, Vector2},
    Color,
};
use acryl_layout::{layout_pager::LayoutPager, node::Node, padding_values::PaddingValues, FONT_DEJAVU_SERIF};
use acryl_parser::{file::DocFile, parse, ParsedFile};
use acryl_pdf::{font::Font, resource::resource_manager::ResourceManager, stream::{FillPaintArgs, FillRule, LineCap, LineJoin, StrokePaintArgs}, structure::Document, write::PdfDocument};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::doc_config::DocumentConfig;

const SAMPLE_FILE_PATH: &str = "examples/minimal.acryl";
const OUT_FILE_PATH: &str = "out/minimal.pdf";

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
    let config: DocumentConfig = doc
        .header()
        .try_into()
        .map_err(|err| panic!("could not parse header '{}'", err))
        .unwrap();

    println!("{:?}", config);

    let mut resource_manager = ResourceManager::new();
    let default_font =
        resource_manager.add_font(Font::load(FONT_DEJAVU_SERIF).expect("Font file not found"));


    let mut page_layout = LayoutPager::new(config.default_page_size);

    {
        let text_node = Node::text("Hello world!", 12.0, default_font.clone());
        
        page_layout.push(text_node)
    }

    // let mut rng = StdRng::from_seed([0; 32]);

    // for _ in 0..20 {
    //     let width = rng.gen_range(50.0..150.0);
    //     let height = rng.gen_range(50.0..150.0);
    //     let color = rng.gen_range(0..0xFFFFFF);

    //     let text_node = Node::text("Hey".to_string(), 12.0, default_font.clone());

    //     let node = text_node.with_size(Vector2::new(width.into(), height.into()));

    //     let node = node.with_color_box(Some(FillPaintArgs {
    //         color: Color::rgb_from_hex(color),
    //         fill_rule: FillRule::EvenOdd,
    //     }), Some(StrokePaintArgs {
    //         close: true,
    //         color: Color::Gray(0),
    //         line_width: Pt(2.0),
    //         line_cap: LineCap::Sqare,
    //         line_join: LineJoin::Bevel,
    //         miter_limit: Pt(10.0),
    //         dash_pattern: (Vec::new(), 0),
    //     })).with_padding(PaddingValues::all(Pt(5.0)));

    //     page_layout.push(node);
    // }

    let pages = page_layout.layout();

    println!("created {} pages", pages.len());

    let pages = pages.into_iter().map(|page| page.paint()).collect();

    let document = Document::new(config.info, resource_manager, pages);
    let mut out_file = File::create(OUT_FILE_PATH).expect("could not create out file");

    let document = PdfDocument::new(document);

    document
        .write(&mut out_file)
        .expect("error while writing document");

    Some(out_file)
}
