use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::vector::VectorComponent;

pub(crate) type UnitValue = f64;

pub trait Unit:
    Copy
    + Clone
    + From<f64>
    + Into<f64>
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
{
}

macro_rules! unit {
    ($name:ident, $format:literal, $($factor:expr => $converted:ty),* $(,)?) => {

#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct $name(pub UnitValue);

impl std::fmt::Display for $name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::fmt::Debug for $name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, concat!("{:.1}", $format), self.0)
    }
}

impl From<f64> for $name {
    fn from(value: UnitValue) -> Self {
        Self(value)
    }
}

impl From<$name> for f64 {
    fn from(value: $name) -> Self {
        value.0
    }
}

impl VectorComponent for $name {
    const ZERO: Self = Self(UnitValue::ZERO);
}
impl Unit for $name {}

// Math operations

impl Add for $name {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for $name {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Sub for $name {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for $name {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0
    }
}

impl Neg for $name {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl Mul for $name {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self(self.0 * rhs.0)
    }
}

impl Mul<UnitValue> for $name {
    type Output = Self;

    fn mul(self, rhs: UnitValue) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl MulAssign for $name {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0
    }
}

impl Div for $name {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self(self.0 / rhs.0)
    }
}

impl DivAssign for $name {
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0
    }
}

$(

impl From<$name> for $converted {
    fn from(value: $name) -> Self {
        Self(value.0 * $factor)
    }
}

)*

    };
}

unit!(Pt, "pt",
    0.3527777778 => Mm,
    0.0352777778 => Cm,
);

unit!(Mm, "mm",
    2.8346456693 => Pt,
    0.1000000000 => Cm,
);

unit!(Cm, "cm",
    10.000000000 => Mm,
    28.346456693 => Pt,
);