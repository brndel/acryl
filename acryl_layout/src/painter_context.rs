use std::fmt::Debug;

use acryl_core::math::{Area, Pt};
use acryl_pdf::stream::Streambuilder;

pub struct PainterContext<'a, 'b> {
    pub stream_builder: &'a mut Streambuilder<'b>,
    pub area: Area<Pt>,
}

impl Debug for PainterContext<'_, '_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PainterContext")
            .field("stream_builder", &"...")
            .field("area", &self.area)
            .finish()
    }
}
