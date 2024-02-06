use std::{cell::RefCell, io::{self, Seek, Write}, rc::Rc};

use crate::{data::PdfObjRef, pdf_dict, structure::Document};

use super::{objects::Objects, pdf_writer::FontContainer, PdfWriter, WritePdf};


pub struct PdfDocument {
    objects: Objects,
    root: PdfObjRef,
    info: PdfObjRef,
}

impl PdfDocument {
    pub fn new(document: Document) -> Self {
        let objects = Rc::new(RefCell::new(Objects::default()));

        let mut writer = PdfWriter::new((), objects.clone());

        let font_container = document.resource_manager.write(&mut writer);

        drop(writer);

        let mut writer = PdfWriter::new(FontContainer { font_container }, objects.clone());

        let root = document.catalog.write(&mut writer);
        let info = writer.add(document.info);

        drop(writer);

        let objects = Rc::try_unwrap(objects).ok().expect("Lost a Rc<Objects> somewhere while writing Document");
        let objects = objects.take();

        return Self {
            objects,
            root,
            info,
        };
    }

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
            "Size" => obj_count,
            "Root" => self.root,
            "Info" => self.info,
        );

        writeln!(f, "trailer")?;
        trailer.render(f)?;

        writeln!(f, "startxref")?;
        writeln!(f, "{}", xrefstart)?;

        writeln!(f, "%%EOF")?;

        Ok(())
    }
    
}