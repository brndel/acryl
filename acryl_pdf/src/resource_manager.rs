use std::{collections::BTreeMap, rc::Rc};

use crate::{data::{PdfObj, PdfObjRef}, font::{Font, FontRef}, write::{PdfWriter, WritePdf}};


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
}

impl<D> WritePdf<D> for ResourceManager {
    fn write(self, writer: &mut PdfWriter<D>) -> PdfObjRef {
        let mut fields = Vec::new();

        for (name, font) in &self.fonts {
            let obj_ref = font.write(writer);
            fields.push((name.to_owned().into(), obj_ref.into()));
        }

        writer.add(PdfObj::Dict(fields))
    }
}