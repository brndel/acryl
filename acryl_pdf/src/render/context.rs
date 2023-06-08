use core::fmt;
use std::{collections::HashMap, fmt::Write};

use super::obj::PdfObj;

#[derive(Default)]
pub struct Context {
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

    pub fn render(self) -> Result<String, fmt::Error> {
        let mut s = String::new();
        writeln!(s, "%PDF-1.7")?;

        struct XRefEntry {
            id: u64,
            generation: u64,
            offset: usize,
        }

        let mut xref: Vec<XRefEntry> = Vec::new();

        xref.push(XRefEntry {
            id: 0,
            generation: 65_535,
            offset: 0,
        }); // "object number 0 shall always be free and shall have a generation number of 65,535"

        let obj_count = self.objects.len();

        for (id, obj) in self.objects {
            xref.push(XRefEntry {
                id,
                generation: 0,
                offset: s.len(),
            });
            writeln!(s, "{} 0 obj", id)?;
            obj.render(&mut s)?;
            writeln!(s, "")?;
            writeln!(s, "endobj")?;
            writeln!(s, "")?;
        }

        xref.sort_by(|a, b| a.id.cmp(&b.id));

        let grouped_xref: Vec<Vec<XRefEntry>> =
            xref.into_iter().fold(Vec::default(), |mut groups, entry| {
                if let Some(last_group) = groups.last_mut() {
                    if let Some(last) = last_group.last_mut() {
                        if last.id + 1 == entry.id  {
                            last_group.push(entry);
                        } else {
                            groups.push(vec![entry]);
                        }
                    } else {
                        last_group.push(entry);
                    }
                } else {
                    groups.push(vec![entry]);
                }

                groups
            });

        let xrefstart = s.len();

        writeln!(s, "xref")?;
        for group in grouped_xref {
            let first = group.first().unwrap();
            writeln!(s, "{} {}", first.id, group.len())?;
            for entry in group {
                writeln!(s, "{:0>10} {:0>5} n", entry.offset, entry.generation)?;
            }
        }

        let trailer = PdfObj::Dict(vec![
            ("Size", obj_count.into()),
            ("Root", self.root.unwrap().into()),
            ("Info", self.info.unwrap().into()),
        ]);

        writeln!(s, "trailer")?;
        trailer.render(&mut s)?;

        writeln!(s, "startxref")?;
        writeln!(s, "{}", xrefstart)?;

        writeln!(s, "%%EOF")?;

        Ok(s)
    }
}
