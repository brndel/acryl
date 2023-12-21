use std::{collections::BTreeMap, rc::Rc};

use crate::{font::{Font, FontRef}, writer::PdfWriter, pdf::{PdfObjRef, PdfObj}};


#[derive(Default)]
pub struct ResourceManager {
    font_name_counter: u64,
    fonts: BTreeMap<String, Rc<Font>>
}



impl ResourceManager {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn add_font(&mut self, font: Font) -> FontRef {
        self.font_name_counter += 1;
        let name = format!("F{}", self.font_name_counter);
        let font = Rc::new(font);
        self.fonts.insert(name.clone(), font.clone());
        FontRef(name, font)
    }

    pub fn render<T: PdfWriter>(&self, writer: &mut T) -> PdfObjRef {
        let mut fields = Vec::new();

        for (name, font) in &self.fonts {
            let obj_ref = font.render(writer);
            fields.push((name.to_owned().into(), obj_ref.into()));
        }

        writer.add(PdfObj::Dict(fields))
    }
}