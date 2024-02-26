use std::ops::Add;

use acryl_core::math::{Vector2, VectorComponent};

pub enum DySize<T: VectorComponent> {
    Fixed(Vector2<T>),
    MixMax {
        min: Vector2<T>,
        max: Vector2<T>,
        preferred: Option<Vector2<T>>,
    },
}

impl<T: VectorComponent> Default for DySize<T> {
    fn default() -> Self {
        Self::Fixed(Vector2::ZERO)
    }
}

impl<T: VectorComponent> DySize<T> {
    pub fn min(&self) -> &Vector2<T> {
        match &self {
            DySize::Fixed(size) => size,
            DySize::MixMax { min: size, .. } => size,
        }
    }

    pub fn max(&self) -> &Vector2<T> {
        match &self {
            DySize::Fixed(size) => size,
            DySize::MixMax { max: size, .. } => size,
        }
    }
}

impl<T: VectorComponent> Add<Vector2<T>> for DySize<T> {
    type Output = Self;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        match self {
            DySize::Fixed(size) => Self::Fixed(size + rhs),
            DySize::MixMax {
                min,
                max,
                preferred,
            } => Self::MixMax {
                min: min + rhs.clone(),
                max: max + rhs.clone(),
                preferred: preferred.map(|preferred| preferred + rhs),
            },
        }
    }
}
