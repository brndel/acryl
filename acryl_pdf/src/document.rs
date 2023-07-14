use core::fmt;
use std::{collections::HashMap, rc::Rc, io::{Write, Seek, self}};

use crate::{
    render::{Context, PdfObjRef, PdfObj},
    unit::Pt,
    util::{constants::PAGE_SIZE_A4, Area, Vector2},
    Page, pdf_dict, font::{ExternalFont, FontRef},
};

#[derive(Default)]
pub struct Document {
    info: DocumentInfo,
    catalog: DocumentCatalog,
    font_counter: u16,
    font_dict: HashMap<String, Rc<ExternalFont>>,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
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

    pub fn add_font(&mut self, font: Rc<ExternalFont>) -> FontRef {
        self.font_counter += 1;
        let name = format!("F{}", self.font_counter); 
        self.font_dict.insert(name.clone(), font);
        let font_ref = FontRef(name);

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
        let obj = pdf_dict!(
            "Title" => PdfObj::StringLiteral(self.title.clone().into()),
            "Author" => PdfObj::StringLiteral(self.author.clone().into()),
            "Subject" => PdfObj::StringLiteral(self.subject.clone().into()),
            "Creator" => PdfObj::StringLiteral("Acryl".into()),
        );

        context.add(obj)
    }
}

#[derive(Default)]
struct DocumentCatalog {
    pages: DocumentPages,
}

impl DocumentCatalog {
    fn render(self, context: &mut Context) -> PdfObjRef {
        let pages = self.pages.render(context);

        let obj = pdf_dict!(
            "Type" => PdfObj::Name("Catalog".into()),
            "Pages" => pages.into(),
        );

        context.add(obj)
    }
}

struct DocumentPages {
    default_page_size: Area<Pt>,
    pages: Vec<Page>,
}

impl Default for DocumentPages {
    fn default() -> Self {
        Self {
            default_page_size: Area::from_size(PAGE_SIZE_A4),
            pages: Vec::default()
        }
    }
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
        self.pages.push(Page::new(
            size.map_or_else(|| self.default_page_size.clone(), |size| Area::from_size(size)),
        ));

        self.pages.last_mut().unwrap()
    }
}
