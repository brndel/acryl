use std::{fmt::Debug, marker::PhantomData, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign}};

use super::coords::{Coords, DefaultCoords};

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
    + Debug
{
    const ZERO: Self;
}

macro_rules! vector_component_impl {
    ($zero:expr, $($name:ty)*) => {
        $(
            impl VectorComponent for $name {
                const ZERO: Self = $zero;
            }
        )*
    };
}

vector_component_impl!(0, u8 u16 u32 u64 i8 i16 i32 i64 usize isize);
vector_component_impl!(0.0, f32 f64);

macro_rules! op_impl {
    ($name:ident, ($op:tt $trait:ident $trait_fn:ident), ($op_assign:tt $trait_assign:ident $assign_fn:ident), $($val:ident,)*) => {

impl<T: VectorComponent, C: Coords> $trait for $name<T, C> {
    type Output = Self;

    fn $trait_fn(self, rhs: Self) -> Self::Output {
        Self {
            $(
                $val: self.$val $op rhs.$val,
            )*
            phantom: Default::default()
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

#[derive(Clone, Default)]
pub struct $name<T: VectorComponent, C: Coords = DefaultCoords> {
    $(
        pub $val: T,
    )+
    phantom: PhantomData<C>
}

impl<T: VectorComponent, C: Coords> $name<T, C> {
    pub const ZERO: Self = Self::all(T::ZERO);

    pub const fn new($($val: T,)*) -> Self {
        Self {
            $($val,)*
            phantom: PhantomData {}
        }
    }

    pub const fn all(value: T) -> Self {
        Self {
            $($val: value,)*
            phantom: PhantomData {}
        }
    }

    pub fn min(self, other: Self) -> Self {
        Self {
            $(
                $val: if self.$val < other.$val { self.$val } else { other.$val },
            )*
            phantom: PhantomData {}
        }
    }

    pub fn max(self, other: Self) -> Self {
        Self {
            $(
                $val: if self.$val > other.$val { self.$val } else { other.$val },
            )*
            phantom: PhantomData {}
        }

    }

    pub fn convert<U: VectorComponent>(self) -> $name<U> where T: Into<U>{
        $name {
            $(
                $val: self.$val.into(),
            )*
            phantom: PhantomData {}
        }
    }

    pub fn scale<M: Copy>(self, value: M) -> Self where T: Mul<M, Output=T> {
        Self {
            $(
                $val: self.$val * value,
            )*
            phantom: PhantomData {}
        }
    }

    pub fn with_coords<R: Coords>(self) -> $name<T, R> {
        $name {
            $(
                $val: self.$val,
            )*
            phantom: PhantomData::<R>,
        }
    }
}

impl<T: VectorComponent, C: Coords> Debug for $name<T, C> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = format!(concat!(stringify!($name), "@{}"), C::name());
        f.debug_tuple(&name)
        $(.field(&self.$val))*
        .finish()
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
            phantom: Default::default()
        }
    }
}

impl<T: VectorComponent> From<T> for $name<T> {
    fn from(value: T) -> Self {
        Self {
            $(
                $val: value,
            )*
            phantom: Default::default()
        }
    }
}

    };
}

vector!(Vector2, (x, y));
vector!(Vector3, (x, y, z));


