use std::{
    collections::HashMap,
    io::{self, Seek, Write},
    rc::Rc,
};

use acryl_core::{Vector2, unit::Pt, Area};

use crate::{
    font::{Font, FontRef},
    pdf_dict,
    render::{Context, PdfObj, PdfObjRef},
    Page,
};

pub struct Document {
    info: DocumentInfo,
    catalog: DocumentCatalog,
    font_counter: u16,
    font_dict: HashMap<String, Rc<Font>>,
}

#[derive(Debug)]
pub struct DocumentConfig {
    pub author: Option<String>,
    pub default_page_size: Vector2<Pt>,
}

impl Document {
    pub fn new(config: DocumentConfig) -> Self {
        Self {
            info: Default::default(),
            catalog: DocumentCatalog {
                pages: DocumentPages {
                    default_page_size: config.default_page_size,
                    pages: Default::default(),
                },
            },
            font_counter: Default::default(),
            font_dict: Default::default(),
        }
    }

    pub fn render<F: Write + Seek>(self, f: &mut F) -> io::Result<()> {
        let mut context = Context::new(self.font_dict);

        let info = self.info.render(&mut context);
        let root = self.catalog.render(&mut context);

        context.set_info(info);
        context.set_root(root);

        context.render(f)
    }

    pub fn add_page(&mut self, size: Option<Vector2<Pt>>) -> &mut Page {
        self.catalog.pages.add_page(size)
    }

    pub fn set_title(&mut self, title: String) {
        self.info.title = title;
    }

    pub fn set_author(&mut self, author: String) {
        self.info.author = author;
    }

    pub fn set_subject(&mut self, subject: String) {
        self.info.subject = subject;
    }

    pub fn add_font(&mut self, font: Font) -> FontRef {
        self.font_counter += 1;
        let name = format!("F{}", self.font_counter);
        let font = Rc::new(font);
        self.font_dict.insert(name.clone(), font.clone());
        let font_ref = FontRef(name, font);

        font_ref
    }
}

#[derive(Default)]
struct DocumentInfo {
    title: String,
    author: String,
    subject: String,
}

impl DocumentInfo {
    fn render(&self, context: &mut Context) -> PdfObjRef {
        pdf_dict!(
            "Title" => PdfObj::StringLiteral(self.title.clone().into()),
            "Author" => PdfObj::StringLiteral(self.author.clone().into()),
            "Subject" => PdfObj::StringLiteral(self.subject.clone().into()),
            "Creator" => PdfObj::StringLiteral("Acryl".into()),
        )
        .add_to(context)
    }
}

struct DocumentCatalog {
    pages: DocumentPages,
}

impl DocumentCatalog {
    fn render(self, context: &mut Context) -> PdfObjRef {
        let pages = self.pages.render(context);

        pdf_dict!(
            "Type" => PdfObj::Name("Catalog".into()),
            "Pages" => pages.into(),
        )
        .add_to(context)
    }
}

struct DocumentPages {
    default_page_size: Vector2<Pt>,
    pages: Vec<Page>,
}

impl DocumentPages {
    fn render(self, context: &mut Context) -> PdfObjRef {
        let obj_ref = context.reserve();

        let mut kids = Vec::new();

        for page in self.pages {
            let id = page.render(context, obj_ref);
            kids.push(id);
        }

        let obj = pdf_dict!(
            "Type" => PdfObj::Name("Pages".into()),
            "Count" => kids.len().into(),
            "Kids" => kids.into(),
        );

        context.insert(obj_ref, obj);

        obj_ref
    }

    fn add_page(&mut self, size: Option<Vector2<Pt>>) -> &mut Page {
        let page_size = size.unwrap_or_else(|| self.default_page_size.clone());
        self.pages.push(Page::new(Area::from_size(page_size)));

        self.pages.last_mut().unwrap()
    }
}
