use std::collections::BTreeMap;

use crate::data::{PdfObj, PdfObjRef};

#[derive(Default)]
pub struct Objects {
    id_counter: u64,
    pub(super) objects: BTreeMap<u64, PdfObj>,
}

impl Objects {
    pub fn add<T: Into<PdfObj>>(&mut self, obj: T) -> PdfObjRef {
        self.id_counter += 1;
        let id = self.id_counter;

        self.objects.insert(id, obj.into());

        PdfObjRef(id)
    }

    pub(super) fn reserve(&mut self) -> PdfObjRef {
        self.id_counter += 1;
        PdfObjRef(self.id_counter)
    }

    pub(super) fn insert_reserved<T: Into<PdfObj>>(
        &mut self,
        obj_ref: PdfObjRef,
        obj: T,
    ) -> PdfObjRef {
        self.objects.insert(obj_ref.0, obj.into());
        obj_ref
    }

    pub fn add_reserved<'a, T: Into<PdfObj>, F: FnOnce(PdfObjRef) -> T + 'a>(
        &'a mut self,
        func: F,
    ) -> PdfObjRef {
        let obj_ref = self.reserve();

        let obj = func(obj_ref);

        self.insert_reserved(obj_ref, obj)
    }
}
