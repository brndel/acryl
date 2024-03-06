
use crate::resource::resource_manager::ResourceManager;

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
}
