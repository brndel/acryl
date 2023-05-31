use std::ops::{Add, Sub, AddAssign, SubAssign};

use crate::render::PdfObj;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pt(i64);

impl Into<PdfObj> for Pt {
    fn into(self) -> PdfObj {
        PdfObj::Int(self.0)
    }
}

impl Add for Pt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Pt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Pt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Pt {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Into<Pt> for i64 {
    fn into(self) -> Pt {
        Pt(self)
    }
}