use std::io::{Write, Seek, self};

use crate::{resource_manager::ResourceManager, writer::PdfDocumentWriter};

use super::{DocumentCatalog, DocumentInfo, Page};

pub struct Document {
    pub(crate) info: DocumentInfo,
    pub(crate) catalog: DocumentCatalog,
    pub(crate) resource_manager: ResourceManager,
}

impl Document {
    pub fn new(info: DocumentInfo, resource_manager: ResourceManager, pages: Vec<Page>) -> Self {
        Self {
            info,
            catalog: DocumentCatalog::new(pages),
            resource_manager,
        }
    }

    pub fn write<F: Write + Seek>(self, f: &mut F) -> io::Result<()> {
        let writer = PdfDocumentWriter::from(self);

        writer.write(f)
    }
}
