use acryl_pdf::{
    unit::Pt,
    util::{Area, VectorComponent},
    Vector2,
};

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

pub enum MainAxisAlignment {
    Start,
    Center,
    End,
    SpaceBetween,
}

impl Default for MainAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}

pub enum CrossAxisAlignment {
    Start,
    Center,
    End,
    Stretch,
}

impl Default for CrossAxisAlignment {
    fn default() -> Self {
        Self::Start
    }
}


impl LayoutElement for LinearLayout {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt> {
        let mut main_size = Default::default();
        let mut cross_size = Default::default();

        for element in &self.elements {
            let element_min_size = element.get_min_size(max_size.clone());
            if self.orientation.get_cross(&element_min_size) > cross_size {
                cross_size = self.orientation.get_cross(&element_min_size);
            }

            main_size += self.orientation.get_main(&element_min_size);
        }

        main_size += self.spacing * Pt((self.elements.len() - 1) as f64);

        self.orientation.create_vector(main_size, cross_size)
    }

    fn render(&self, area: Area<Pt>, builder: &mut acryl_pdf::stream::Streambuilder) {
        let min_size = self.get_min_size(area.size.clone());
        let remaining_main_space = self.orientation.get_main(&area.size) - self.orientation.get_main(&min_size);

        let mut main_pos = match &self.main_axis {
            MainAxisAlignment::Start => Pt(0.0),
            MainAxisAlignment::Center => remaining_main_space / Pt(2.0),
            MainAxisAlignment::End => remaining_main_space,
            MainAxisAlignment::SpaceBetween => Pt(0.0),
        };

        for element in &self.elements {
            let min_size = element.get_min_size(area.size.clone());
            let remaining_cross_space = self.orientation.get_cross(&area.size)
                - self.orientation.get_cross(&min_size);

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

            main_pos += main_size + self.spacing;
            if let MainAxisAlignment::SpaceBetween = self.main_axis {
                main_pos += remaining_main_space / Pt((self.elements.len() - 1) as f64);
            }
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
}
