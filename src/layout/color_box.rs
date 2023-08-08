use acryl_core::{Vector2, unit::Pt, Area, Color};

use super::{LayoutBox, LayoutElement};

pub struct ColorBox {
    element: LayoutBox,
    color: Color,
}

impl LayoutElement for ColorBox {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        self.element.get_min_size(max_size)
    }

    fn render(&self, area: Area<Pt>, builder: &mut acryl_pdf::stream::Streambuilder) {
        builder.draw_rect(area.clone(), self.color.clone());
        self.element.render(area, builder);
    }
}

impl ColorBox {
    pub fn new<T: LayoutElement + 'static>(element: T, color: Color) -> Self {
        Self {
            element: Box::new(element),
            color,
        }
    }
}
