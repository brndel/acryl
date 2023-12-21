use super::PdfObj;

#[derive(Clone, Copy)]
pub struct PdfObjRef(pub(crate) u64);

impl From<PdfObjRef> for PdfObj {
    fn from(value: PdfObjRef) -> Self {
        PdfObj::Refernce(value.0, 0)
    }
}