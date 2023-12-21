use std::{
    collections::BTreeMap,
    io::{self, Seek, Write},
};

use crate::{
    pdf::{PdfObj, PdfObjRef, structure::Document},
    pdf_dict,
};

pub trait PdfWriter {
    fn add<T: Into<PdfObj>>(&mut self, obj: T) -> PdfObjRef;
    fn reserve(&mut self) -> PdfObjRef;
    fn insert<T: Into<PdfObj>>(&mut self, obj_ref: PdfObjRef, obj: T);
}

pub struct PdfDocumentWriter {
    objects: Objects,
    root: PdfObjRef,
    info: PdfObjRef,
}

#[derive(Default)]
struct Objects {
    id_counter: u64,
    objects: BTreeMap<u64, PdfObj>,
}

impl PdfWriter for Objects {
    fn add<T: Into<PdfObj>>(&mut self, obj: T) -> PdfObjRef {
        self.id_counter += 1;
        let id = self.id_counter;

        self.objects.insert(id, obj.into());

        PdfObjRef(id)
    }

    fn reserve(&mut self) -> PdfObjRef {
        self.id_counter += 1;

        PdfObjRef(self.id_counter)
    }

    fn insert<T: Into<PdfObj>>(&mut self, obj_ref: PdfObjRef, obj: T) {
        self.objects.insert(obj_ref.0, obj.into());
    }
}

impl PdfWriter for PdfDocumentWriter {
    fn add<T: Into<PdfObj>>(&mut self, obj: T) -> PdfObjRef {
        self.objects.add(obj)
    }

    fn reserve(&mut self) -> PdfObjRef {
        self.objects.reserve()
    }

    fn insert<T: Into<PdfObj>>(&mut self, obj_ref: PdfObjRef, obj: T) {
        self.objects.insert(obj_ref, obj)
    }
}

impl From<Document> for PdfDocumentWriter {
    fn from(value: Document) -> Self {
        let mut objects = Objects::default();

        let font_container = value.resource_manager.render(&mut objects);

        let root = value.catalog.render(&mut objects, font_container);
        let info = objects.add(value.info);

        return Self {
            objects,
            root,
            info,
        };
    }
}

impl PdfDocumentWriter {
    pub fn write<F: Write + Seek>(self, f: &mut F) -> io::Result<()> {
        writeln!(f, "%PDF-1.7")?;

        struct XRefEntry {
            id: u64,
            generation: u64,
            offset: u64,
        }

        let mut xref: Vec<XRefEntry> = Vec::new();

        xref.push(XRefEntry {
            id: 0,
            generation: 65_535,
            offset: 0,
        }); // "object number 0 shall always be free and shall have a generation number of 65,535"

        let objects = self.objects.objects;

        let obj_count = objects.len();

        for (id, obj) in objects {
            xref.push(XRefEntry {
                id,
                generation: 0,
                offset: f.stream_position()?,
            });
            writeln!(f, "{} 0 obj", id)?;
            obj.render(f)?;
            writeln!(f, "")?;
            writeln!(f, "endobj")?;
            writeln!(f, "")?;
        }

        // xref.sort_by(|a, b| a.id.cmp(&b.id));

        let grouped_xref: Vec<Vec<XRefEntry>> =
            xref.into_iter().fold(Vec::default(), |mut groups, entry| {
                if let Some(last_group) = groups.last_mut() {
                    if let Some(last) = last_group.last_mut() {
                        if last.id + 1 == entry.id {
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

        let xrefstart = f.stream_position()?;

        writeln!(f, "xref")?;
        for group in grouped_xref {
            let first = group.first().unwrap();
            writeln!(f, "{} {}", first.id, group.len())?;
            for entry in group {
                writeln!(f, "{:0>10} {:0>5} n", entry.offset, entry.generation)?;
            }
        }

        let trailer = pdf_dict!(
            "Size" => obj_count.into(),
            "Root" => self.root.into(),
            "Info" => self.info.into(),
        );

        writeln!(f, "trailer")?;
        trailer.render(f)?;

        writeln!(f, "startxref")?;
        writeln!(f, "{}", xrefstart)?;

        writeln!(f, "%%EOF")?;

        Ok(())
    }
}
