use crate::data::{PdfObj, PdfObjRef};

use super::pdf_writer::{PdfWriter, PdfWriterDefaultData};


pub trait WritePdf<D = PdfWriterDefaultData> {
    fn write(self, writer: &mut PdfWriter::<D>) -> PdfObjRef;
}

impl<T: Into<PdfObj>, D> WritePdf<D> for T {
    fn write(self, writer: &mut PdfWriter::<D>) -> PdfObjRef {
        writer.add(self)
    }
}
