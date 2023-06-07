use std::ops::Add;

use crate::render::PdfObj;

use super::Pt;

#[derive(Clone, Default)]
pub struct Vector2 {
    pub x: Pt,
    pub y: Pt,
}

impl Into<PdfObj> for Vector2 {
    fn into(self) -> PdfObj {
        PdfObj::Array(vec![self.x.into(), self.y.into()])
    }
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
