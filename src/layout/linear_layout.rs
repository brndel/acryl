use acryl_core::{math::{Pt, Area, Vector2}, CrossAxisAlignment, MainAxisAlignment};

use crate::util::orientation::Orientation;

use super::{LayoutBox, LayoutElement};

#[derive(Default)]
pub struct LinearLayout {
    elements: Vec<LayoutBox>,
    orientation: Orientation,
    main_axis: MainAxisAlignment,
    cross_axis: CrossAxisAlignment,
    spacing: Pt,
}

impl LayoutElement for LinearLayout {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        let result = self.get_min_size_of_elements(max_size);

        self.orientation.create_vector(result.main_size, result.cross_size)
    }

    fn render(&self, area: Area<Pt>, builder: &mut acryl_pdf::stream::Streambuilder) {
        let result = self.get_min_size_of_elements(area.size.clone());

        let positions = self.main_axis.get_positions(self.orientation.get_main(&area.size), &result.element_sizes);

        for (element, main_pos) in self.elements.iter().zip(positions) {
            let min_size = element.get_min_size(area.size.clone());
            let remaining_cross_space =
                self.orientation.get_cross(&area.size) - self.orientation.get_cross(&min_size);

            let cross_pos = match &self.cross_axis {
                CrossAxisAlignment::Center => remaining_cross_space / Pt(2.0),
                CrossAxisAlignment::End => remaining_cross_space,
                _ => Pt(0.0),
            };

            let cross_size = match &self.cross_axis {
                CrossAxisAlignment::Stretch => self.orientation.get_cross(&area.size),
                _ => self.orientation.get_cross(&min_size),
            };

            let main_size = self.orientation.get_main(&min_size);

            element.render(
                Area {
                    position: area.position.clone()
                        + self.orientation.create_vector(main_pos, cross_pos),
                    size: self.orientation.create_vector(main_size, cross_size),
                },
                builder,
            );

        }
    }
}

impl LinearLayout {
    pub fn vertical() -> Self {
        Self {
            orientation: Orientation::Vertical,
            ..Default::default()
        }
    }

    pub fn horizontal() -> Self {
        Self {
            orientation: Orientation::Horizontal,
            ..Default::default()
        }
    }

    pub fn main_axis(mut self, main_axis: MainAxisAlignment) -> Self {
        self.main_axis = main_axis;
        self
    }

    pub fn cross_axis(mut self, cross_axis: CrossAxisAlignment) -> Self {
        self.cross_axis = cross_axis;
        self
    }

    pub fn spacing(mut self, spacing: Pt) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn add<T: LayoutElement + 'static>(&mut self, element: T) {
        self.elements.push(Box::new(element));
    }

    fn get_min_size_of_elements(&self, max_size: Vector2<Pt>) -> MinSizeResult {
        let mut main_size = Pt(0.0);
        let mut cross_size = Pt(0.0);
        let mut element_sizes = Vec::new();
        for elem in &self.elements {
            let size = elem.get_min_size(max_size.clone());
            let main = self.orientation.get_main(&size);
            let cross = self.orientation.get_cross(&size);

            main_size += main;
            if cross > cross_size {
                cross_size = cross;
            }
            element_sizes.push(main);
        }
        MinSizeResult {
            main_size,
            cross_size,
            element_sizes,
        }
    }
}

struct MinSizeResult {
    main_size: Pt,
    cross_size: Pt,
    element_sizes: Vec<Pt>,
}
