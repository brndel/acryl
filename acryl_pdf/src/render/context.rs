use std::{collections::HashMap, io::{Write, self, Seek}, rc::Rc};

use crate::{font::ExternalFont, pdf_dict};

use super::obj::PdfObj;

#[derive(Default)]
pub struct Context {
    id_counter: u64,
    root: Option<PdfObjRef>,
    info: Option<PdfObjRef>,
    objects: HashMap<u64, PdfObj>,
    font_map: HashMap<String, PdfObjRef>,
    font_object: Option<PdfObjRef>,
}

#[derive(Clone, Copy)]
pub struct PdfObjRef(u64);

impl Into<PdfObj> for PdfObjRef {
    fn into(self) -> PdfObj {
        PdfObj::Refernce(self.0, 0)
    }
}

impl Context {
    pub fn new(fonts: HashMap<String, Rc<ExternalFont>>) -> Self {
        let mut this = Self::default();
        let mut font_object_fields = Vec::default();

        for (font_name, font) in fonts {
            let id = font.render(&mut this);
            this.font_map.insert(font_name.clone(), id);
            font_object_fields.push((font_name.into(), id.into()));
        }

        let font_obj = this.add(PdfObj::Dict(font_object_fields));

        this.font_object = Some(font_obj);

        return this;
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

    pub fn get_font_obj(&self) -> PdfObjRef {
        self.font_object.unwrap()
    }

    pub fn render<F: Write + Seek>(self, f: &mut F) -> io::Result<()> {
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

        let obj_count = self.objects.len();

        for (id, obj) in self.objects {
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

        xref.sort_by(|a, b| a.id.cmp(&b.id));

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
            "Root" => self.root.unwrap().into(),
            "Info" => self.info.unwrap().into(),
        );

        writeln!(f, "trailer")?;
        trailer.render(f)?;

        writeln!(f, "startxref")?;
        writeln!(f, "{}", xrefstart)?;

        writeln!(f, "%%EOF")?;

        Ok(())
    }
}
