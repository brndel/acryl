use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::{render::PdfObj, util::vec::VectorComponent};

use super::Pt;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct Mm(pub f64);

impl Display for Mm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Mm> for f64 {
    fn into(self) -> Mm {
        Mm(self)
    }
}

// Vector Component

impl VectorComponent for Mm {}

impl Into<PdfObj> for Mm {
    fn into(self) -> PdfObj {
        let pt: Pt = self.into();
        pt.into()
    }
}

impl Add for Mm {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Mm {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for Mm {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Mm {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

// Unit converters

impl Into<Pt> for Mm {
    fn into(self) -> Pt {
        Pt(self.0 * 2.8346456693)
    }
}
