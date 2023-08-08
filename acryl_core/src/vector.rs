use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

pub trait VectorComponent:
    Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Default
    + Copy
    + PartialOrd
    + PartialEq
{
}

macro_rules! vector_component_impl {
    ($($name:ty)*) => {
        $(
            impl VectorComponent for $name {}
        )*
    };
}

vector_component_impl!(u8 u16 u32 u64 i8 i16 i32 i64 usize isize f32 f64);

macro_rules! op_impl {
    ($name:ident, ($op:tt $trait:ident $trait_fn:ident), ($op_assign:tt $trait_assign:ident $assign_fn:ident), $($val:ident,)*) => {

impl<T: VectorComponent> $trait for $name<T> {
    type Output = Self;

    fn $trait_fn(self, rhs: Self) -> Self::Output {
        Self {
            $(
                $val: self.$val $op rhs.$val,
            )*
        }
    }
}

impl<T: VectorComponent> $trait_assign for $name<T> {
    fn $assign_fn(&mut self, rhs: Self) {
        $(
            self.$val $op_assign rhs.$val;
        )*
    }
}

    };
}

macro_rules! vector {
    ($name:ident, ($($val:ident),+)) => {

#[derive(Clone, Default, Debug)]
pub struct $name<T: VectorComponent> {
    $(
        pub $val: T,
    )+
}

impl<T: VectorComponent> $name<T> {
    pub fn min(self, other: Self) -> Self {
        Self {
            $(
                $val: if self.$val < other.$val { self.$val } else { other.$val },
            )*
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            $(
                $val: if self.$val > other.$val { self.$val } else { other.$val },
            )*
        }

    }

    pub fn convert<U: VectorComponent>(self) -> $name<U> where T: Into<U>{
        $name {
            $(
                $val: self.$val.into(),
            )*
        }
    }
}

op_impl!($name, (+ Add add), (+= AddAssign add_assign), $($val,)*);
op_impl!($name, (- Sub sub), (-= SubAssign sub_assign), $($val,)*);
op_impl!($name, (* Mul mul), (*= MulAssign mul_assign), $($val,)*);

impl<T: VectorComponent> Mul<T> for $name<T> {
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Self {
            $(
                $val: self.$val * rhs,
            )*
        }
    }
}

impl<T: VectorComponent> From<T> for $name<T> {
    fn from(value: T) -> Self {
        Self {
            $(
                $val: value,
            )*
        }
    }
}

    };
}

vector!(Vector2, (x, y));
vector!(Vector3, (x, y, z));
