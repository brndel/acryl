use core::fmt;

use crate::{
    element::PageElement,
    render::{Context, PdfObj, PdfObjRef},
    util::{Area, Vector2},
};

#[derive(Default)]
pub struct Document {
    info: DocumentInfo,
    catalog: DocumentCatalog,
}

impl Document {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render(&self) -> Result<String, fmt::Error> {
        let mut context = Context::new();

        let info = self.info.render(&mut context);
        let root = self.catalog.render(&mut context);

        context.set_info(info);
        context.set_root(root);

        context.render()
    }

    pub fn add_page(&mut self, size: Vector2) -> &mut Page {
        self.catalog.pages.add_page(size)
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
        let obj = PdfObj::Dict(vec![
            ("Title", PdfObj::StringLiteral(self.title.clone())),
            ("Author", PdfObj::StringLiteral(self.author.clone())),
            ("Subject", PdfObj::StringLiteral(self.subject.clone())),
            ("Creator", PdfObj::StringLiteral("Acryl".into())),
        ]);

        context.add(obj)
    }
}

#[derive(Default)]
struct DocumentCatalog {
    pages: DocumentPages,
}

impl DocumentCatalog {
    fn render(&self, context: &mut Context) -> PdfObjRef {
        let pages = self.pages.render(context);
        let obj = PdfObj::Dict(vec![
            ("Type", PdfObj::Name("Catalog")),
            ("Pages", pages.into()),
        ]);

        context.add(obj)
    }
}

#[derive(Default)]
struct DocumentPages {
    pages: Vec<Page>,
}

impl DocumentPages {
    fn render(&self, context: &mut Context) -> PdfObjRef {
        let obj_ref = context.reserve();

        let mut kids = Vec::new();

        for page in &self.pages {
            let id = page.render(context, obj_ref);
            kids.push(id);
        }

        let obj = PdfObj::Dict(vec![
            ("Type", PdfObj::Name("Pages".into())),
            ("Count", kids.len().into()),
            ("Kids", kids.into()),
        ]);

        context.insert(obj_ref, obj);

        obj_ref
    }

    fn add_page(&mut self, size: Vector2) -> &mut Page {
        self.pages.push(Page { area: Area::from_size(size), elements: Vec::new() });

        self.pages.last_mut().unwrap()
    }
}

pub struct Page {
    area: Area,
    elements: Vec<Box<dyn PageElement>>,
}

impl Page {
    fn render(&self, context: &mut Context, parent: PdfObjRef) -> PdfObjRef {
        let mut content = Vec::new();

        for element in &self.elements {
            let id = element.render(context, &self.area);
            content.push(id);
        }

        let obj = PdfObj::Dict(vec![
            ("Type", PdfObj::Name("Page".into())),
            ("Parent", parent.into()),
            ("MediaBox", self.area.clone().into()),
            ("Contents", content.into()),
        ]);

        context.add(obj)
    }
}
