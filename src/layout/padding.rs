use acryl_pdf::{unit::Pt, util::Area, Vector2};

use super::{LayoutBox, LayoutElement};

pub struct Padding {
    element: LayoutBox,
    padding: Pt,
}

impl LayoutElement for Padding {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> acryl_pdf::Vector2<Pt> {
        self.element.get_min_size(max_size) + Vector2::from(self.padding * Pt(2.0))
    }

    fn render(
        &self,
        area: acryl_pdf::util::Area<Pt>,
        builder: &mut acryl_pdf::stream::Streambuilder,
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
