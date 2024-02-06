use std::{cell::RefCell, rc::Rc};

use crate::data::{PdfObj, PdfObjRef};

use super::objects::Objects;

pub type PdfWriterDefaultData = FontContainer;

pub struct PdfWriter<D = PdfWriterDefaultData> {
    data: D,
    parent: Option<PdfObjRef>,
    objects: Rc<RefCell<Objects>>,
}

impl<D> PdfWriter<D> {
    pub fn new(data: D, objects: Rc<RefCell<Objects>>) -> Self {
        Self {
            data,
            objects,
            parent: None,
        }
    }

    pub fn add<T: Into<PdfObj>>(&mut self, obj: T) -> PdfObjRef {
        self.objects.borrow_mut().add(obj)
    }

    pub fn add_reserved<T: Into<PdfObj>, F: FnOnce(&mut Self) -> T>(&mut self, f: F) -> PdfObjRef where D: Copy {
        let obj_ref = self.reserve();

        let obj = f(&mut self.with_parent(obj_ref));

        self.insert_reserved(obj_ref, obj)
    }

    fn reserve(&mut self) -> PdfObjRef {
        self.objects.borrow_mut().reserve()
    }

    fn insert_reserved<T: Into<PdfObj>>(&mut self, obj_ref: PdfObjRef, obj: T) -> PdfObjRef {
        self.objects.borrow_mut().insert_reserved(obj_ref, obj)
    }

    fn with_parent(&self, parent: PdfObjRef) -> Self where D: Copy {
        Self {
            data: self.data,
            parent: Some(parent),
            objects: self.objects.clone(),
        }
    }

    pub fn parent(&self) -> Option<PdfObjRef> {
        self.parent
    }
}

impl PdfWriter<FontContainer> {
    pub fn font_container(&self) -> PdfObjRef {
        self.data.font_container
    }
}

#[derive(Clone, Copy)]
pub struct FontContainer {
    pub font_container: PdfObjRef,
}
