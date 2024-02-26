use crate::math::{VectorComponent, Vector2};

#[derive(Debug, Default, Clone, Copy)]
pub enum Orientation {
    #[default]
    Vertical,
    Horizontal,
}

impl Orientation {
    pub const fn create_vector<T: VectorComponent>(&self, main: T, cross: T) -> Vector2<T> {
        match self {
            Orientation::Vertical => Vector2::new(cross, main),
            Orientation::Horizontal => Vector2::new(main, cross),
        }
    }

    pub const fn get_main<T: VectorComponent>(&self, v: &Vector2<T>) -> T {
        match self {
            Orientation::Vertical => v.y,
            Orientation::Horizontal => v.x,
        }
    }

    pub const fn get_cross<T: VectorComponent>(&self, v: &Vector2<T>) -> T {
        match self {
            Orientation::Vertical => v.x,
            Orientation::Horizontal => v.y,
        }
    }
}