use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::render::PdfObj;

pub trait VectorComponent: Add<Output = Self> + AddAssign + Sub<Output = Self> + SubAssign + Into<PdfObj> + Default + Copy {}
pub trait Vector<T: VectorComponent>: Add + Sub + Into<PdfObj> {}

impl VectorComponent for isize {}
impl VectorComponent for i8 {}
impl VectorComponent for i16 {}
impl VectorComponent for i32 {}
impl VectorComponent for i64 {}

impl VectorComponent for usize {}
impl VectorComponent for u8 {}
impl VectorComponent for u16 {}
impl VectorComponent for u32 {}
impl VectorComponent for u64 {}

impl VectorComponent for f32 {}
impl VectorComponent for f64 {}

#[derive(Clone, Default, Debug)]
pub struct Vector2<T: VectorComponent> {
    pub x: T,
    pub y: T,
}

impl<T: VectorComponent> Into<PdfObj> for Vector2<T> {
    fn into(self) -> PdfObj {
        PdfObj::Array(vec![self.x.into(), self.y.into()])
    }
}

impl<T: VectorComponent> Add for Vector2<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: VectorComponent> AddAssign for Vector2<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: VectorComponent> Sub for Vector2<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: VectorComponent> SubAssign for Vector2<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: VectorComponent> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value }
    }
}