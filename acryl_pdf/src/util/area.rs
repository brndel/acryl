use crate::render::PdfObj;

use super::{Vector2, vec::VectorComponent};

#[derive(Clone)]
pub struct Area<T: VectorComponent> {
    pub position: Vector2<T>,
    pub size: Vector2<T>,
}

impl<T: VectorComponent> Into<PdfObj> for Area<T> {
    fn into(self) -> PdfObj {
        vec![
            self.position.x,
            self.position.y,
            self.position.x + self.size.x,
            self.position.y + self.size.y,
        ]
        .into()
    }
}

impl<T: VectorComponent> Area<T> {
    pub fn from_size(size: Vector2<T>) -> Self {
        Self {
            position: Vector2::default(),
            size,
        }
    }

    pub fn from_points(top_left: Vector2<T>, bottom_right: Vector2<T>) -> Area<T> {
        Self {
            position: top_left,
            size: bottom_right - top_left,
        }
    }

    pub fn top_left(&self) -> Vector2<T> {
        self.position
    }

    pub fn top_right(&self) -> Vector2<T> {
        self.position
            + Vector2 {
                x: self.size.x,
                y: T::default(),
            }
    }

    pub fn bottom_left(&self) -> Vector2<T> {
        self.position
            + Vector2 {
                x: T::default(),
                y: self.size.y,
            }
    }

    pub fn bottom_right(&self) -> Vector2<T> {
        self.position + self.size
    }
}
