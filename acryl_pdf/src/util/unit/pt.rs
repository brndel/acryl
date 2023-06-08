use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{render::PdfObj, util::vec::VectorComponent};

use super::Mm;

#[derive(Clone, Copy, Debug, Default)]
pub struct Pt(pub f64);

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Pt> for f64 {
    fn into(self) -> Pt {
        Pt(self)
    }
}

// Vector Component

impl VectorComponent for Pt {}

impl Into<PdfObj> for Pt {
    fn into(self) -> PdfObj {
        PdfObj::Float(self.0)
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

// Unit converters

impl Into<Mm> for Pt {
    fn into(self) -> Mm {
        Mm(self.0 * 0.3527777778)
    }
}