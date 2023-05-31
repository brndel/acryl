use core::fmt;
use std::{collections::HashMap, fmt::Write};

use super::obj::PdfObj;

#[derive(Default)]
pub(crate) struct Context {
    id_counter: u64,
    root: Option<PdfObjRef>,
    info: Option<PdfObjRef>,
    objects: HashMap<u64, PdfObj>,
}

#[derive(Clone, Copy)]
pub struct PdfObjRef(u64);

impl Into<PdfObj> for PdfObjRef {
    fn into(self) -> PdfObj {
        PdfObj::Refernce(self.0, 0)
    }
}

impl Context {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, obj: PdfObj) -> PdfObjRef {
        self.id_counter += 1;
        let id = self.id_counter;

        self.objects.insert(id, obj);

        PdfObjRef(id)
    }

    pub fn reserve(&mut self) -> PdfObjRef {
        self.id_counter += 1;

        PdfObjRef(self.id_counter)
    }

    pub fn insert(&mut self, obj_ref: PdfObjRef, obj: PdfObj) {
        self.objects.insert(obj_ref.0, obj);
    }

    pub fn set_root(&mut self, obj_ref: PdfObjRef) {
        self.root = Some(obj_ref);
    }

    pub fn set_info(&mut self, obj_ref: PdfObjRef) {
        self.info = Some(obj_ref);
    }

    pub fn render(&self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        writeln!(s, "%PDF-1.7")?;
        

        let mut xref: Vec<(u64, u64, u64)> = Vec::new();

        xref.push((0, 65_535, 0)); // "object number 0 shall always be free and shall have a generation number of 65,535"

        for (id, obj) in &self.objects {
            xref.push((*id, 0, s.len() as u64));
            writeln!(s, "{} 0 obj", id)?;
            writeln!(s, "{}", obj)?;
            writeln!(s, "endobj")?;
            writeln!(s, "")?;
        }

        let xrefstart = s.len();

        writeln!(s, "xref")?;
        for (id, generation, offset) in xref {
            writeln!(s, "{} 1", id)?;
            writeln!(s, "{:0>10} {:0>5} n", offset, generation)?;
        }

        let trailer = PdfObj::Dict(vec![
            ("Size", self.objects.len().into()),
            ("Root", self.root.unwrap().into()),
            ("Info", self.info.unwrap().into()),
        ]);

        writeln!(s, "trailer")?;
        writeln!(s, "{}", trailer)?;

        writeln!(s, "startxref")?;
        writeln!(s, "{}", xrefstart)?;

        writeln!(s, "%%EOF")?;

        Ok(s)
    }
}
