use std::str::FromStr;

use acryl_core::{unit::{Mm, Pt}, Vector2};


pub enum PageSize {
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
}

impl Default for PageSize {
    fn default() -> Self {
        Self::A4
    }
}

#[derive(Debug)]
pub struct PageSizeUnknown;

impl FromStr for PageSize {
    type Err = PageSizeUnknown;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A0" => Ok(Self::A0),
            "A1" => Ok(Self::A1),
            "A2" => Ok(Self::A2),
            "A3" => Ok(Self::A3),
            "A4" => Ok(Self::A4),
            "A5" => Ok(Self::A5),
            "A6" => Ok(Self::A6),
            "A7" => Ok(Self::A7),
            "A8" => Ok(Self::A8),
            _ => Err(PageSizeUnknown),
        }
    }

}

impl PageSize {
    pub fn get_size(&self) -> Vector2<Pt> {
        match self {
            PageSize::A0 => Vector2 { x: Mm(841.00), y: Mm(1189.00) },
            PageSize::A1 => Vector2 { x: Mm(594.0) , y: Mm(841.0)},
            PageSize::A2 => Vector2 { x: Mm(420.0) , y: Mm(594.0)},
            PageSize::A3 => Vector2 { x: Mm(297.0) , y: Mm(420.0)},
            PageSize::A4 => Vector2 { x: Mm(210.0) , y: Mm(297.0)},
            PageSize::A5 => Vector2 { x: Mm(148.0) , y: Mm(210.0)},
            PageSize::A6 => Vector2 { x: Mm(105.0) , y: Mm(148.0)},
            PageSize::A7 => Vector2 { x: Mm(74.0) , y: Mm(105.0)},
            PageSize::A8 => Vector2 { x: Mm(52.0) , y: Mm(74.0)},
        }.convert()
    }
}