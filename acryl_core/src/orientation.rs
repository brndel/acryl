use crate::math::{VectorComponent, Vector2};

pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Default for Orientation {
    fn default() -> Self {
        Self::Vertical
    }
}

impl Orientation {
    pub fn create_vector<T: VectorComponent>(&self, main: T, cross: T) -> Vector2<T> {
        match self {
            Orientation::Vertical => Vector2::new(cross, main),
            Orientation::Horizontal => Vector2::new(main, cross),
        }
    }

    pub fn get_main<T: VectorComponent>(&self, v: &Vector2<T>) -> T {
        match self {
            Orientation::Vertical => v.y,
            Orientation::Horizontal => v.x,
        }
    }

    pub fn get_cross<T: VectorComponent>(&self, v: &Vector2<T>) -> T {
        match self {
            Orientation::Vertical => v.x,
            Orientation::Horizontal => v.y,
        }
    }
}