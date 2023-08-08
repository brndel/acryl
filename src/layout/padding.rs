use acryl_core::{Vector2, unit::Pt, Area};
use acryl_pdf::stream::Streambuilder;

use super::{LayoutBox, LayoutElement};

pub struct Padding {
    element: LayoutBox,
    padding: Pt,
}

impl LayoutElement for Padding {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        self.element.get_min_size(max_size) + Vector2::from(self.padding * Pt(2.0))
    }

    fn render(
        &self,
        area: Area<Pt>,
        builder: &mut Streambuilder,
    ) {
        let area = Area {
            position: area.position + Vector2::from(self.padding),
            size: area.size - Vector2::from(self.padding * Pt(2.0)),
        };
        self.element.render(area, builder);
    }
}

impl Padding {
    pub fn new<T: LayoutElement + 'static>(element: T, padding: Pt) -> Self {
        Self {
            element: Box::new(element),
            padding,
        }
    }
}
