use std::{collections::BTreeMap, rc::Rc};

use crate::{
    data::{PdfObj, PdfObjRef},
    font::Font,
    write::{PdfWriter, WritePdf},
};

#[derive(Default)]
pub struct ResourceManager {
    font_name_counter: u64,
    data: BTreeMap<String, Resource>,
}

enum Resource {
    Font(Rc<Font>),
}

#[derive(Debug)]
pub enum ResourceError {
    NoResource,
    WrongType,
}

impl ResourceManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_font(&mut self, font: Font) -> ResourceRef<Font> {
        self.font_name_counter += 1;
        let name = format!("F{}", self.font_name_counter);
        self.data.insert(name.clone(), Resource::Font(Rc::new(font)));

        self.get_font(&name).unwrap()
    }

    pub fn get_font<'a>(&'a self, name: &str) -> Result<ResourceRef<Font>, ResourceError> {
        let (name, resource) = self
            .data
            .get_key_value(name)
            .ok_or(ResourceError::NoResource)?;
        ResourceRef::new(name.to_owned(), resource).ok_or(ResourceError::WrongType)
    }
}

pub struct ResourceRef<T> {
    name: String,
    data: Rc<T>,
}

impl<T> Clone for ResourceRef<T> {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), data: self.data.clone() }
    }
}

impl<T> ResourceRef<T> {
    fn new<'a>(name: String, resource: &'a Resource) -> Option<Self>
    where
        &'a Resource: TryInto<Rc<T>>,
    {
        Some(Self {
            name,
            data: resource.try_into().ok()?,
        })
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn data_rc(&self) -> Rc<T> {
        self.data.clone()
    }

    pub fn data(&self) -> &T {
        &self.data
    }
}

impl<D> WritePdf<D> for ResourceManager {
    fn write(self, writer: &mut PdfWriter<D>) -> PdfObjRef {
        let mut fields = Vec::new();

        for (name, font) in self.data {
            let obj_ref = font.write(writer);
            fields.push((name.to_owned().into(), obj_ref.into()));
        }

        writer.add(PdfObj::Dict(fields))
    }
}

impl<'a> TryInto<Rc<Font>> for &'a Resource {
    type Error = ();

    fn try_into(self) -> Result<Rc<Font>, Self::Error> {
        match self {
            Resource::Font(font) => Ok(font.clone()),
            // _ => Err(()),
        }
    }
}

impl<D> WritePdf<D> for Resource {
    fn write(self, writer: &mut PdfWriter<D>) -> PdfObjRef {
        match self {
            Resource::Font(font) => font.pdf_font().write(writer),
        }
    }
}
