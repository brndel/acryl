use acryl_core::math::{Vector2, Pt, Area};
use acryl_pdf::stream::Streambuilder;

use super::LayoutElement;

pub struct SizeBox {
    size: Vector2<Pt>,
}

impl LayoutElement for SizeBox {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        self.size.clone().min(max_size)
    }

    fn render(&self, _area: Area<Pt>, _builder: &mut Streambuilder) {}
}

impl SizeBox {
    pub fn new<X: Into<Pt>, Y: Into<Pt>>(x: X, y: Y) -> Self {
        Self { size: Vector2::new(x.into(), y.into()) }
    }
}