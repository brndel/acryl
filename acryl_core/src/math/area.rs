use crate::math::{
    coords::Coords,
    vector::{Vector2, VectorComponent},
};

use super::DefaultCoords;

#[derive(Debug, Clone)]
pub struct Area<T: VectorComponent, C: Coords = DefaultCoords> {
    pub position: Vector2<T, C>,
    pub size: Vector2<T, C>,
}

impl<T: VectorComponent, C: Coords> Area<T, C> {
    pub fn from_size(size: Vector2<T, C>) -> Self {
        Self {
            position: Vector2::default(),
            size,
        }
    }

    pub fn from_points(top_left: Vector2<T, C>, bottom_right: Vector2<T, C>) -> Self {
        Self {
            position: top_left.clone(),
            size: bottom_right - top_left,
        }
    }

    pub fn top_left(&self) -> Vector2<T, C> {
        Vector2::new(self.left(), self.top())
    }

    pub fn top_right(&self) -> Vector2<T, C> {
        Vector2::new(self.right(), self.top())
    }

    pub fn bottom_left(&self) -> Vector2<T, C> {
        Vector2::new(self.left(), self.bottom())
    }

    pub fn bottom_right(&self) -> Vector2<T, C> {
        Vector2::new(self.right(), self.bottom())
    }

    pub fn top(&self) -> T {
        self.position.y
    }

    pub fn bottom(&self) -> T {
        self.position.y + self.size.y
    }
    pub fn left(&self) -> T {
        self.position.x
    }

    pub fn right(&self) -> T {
        self.position.x + self.size.x
    }

    pub fn with_coords<R: Coords>(self) -> Area<T, R> {
        Area {
            position: self.position.with_coords::<R>(),
            size: self.size.with_coords::<R>(),
        }
    }
}
