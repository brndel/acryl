use acryl_core::Color;

use crate::render::PdfObj;

use super::StreamInstruction;

pub enum ColorOperation {
    StrokeColor(Color),
    FillColor(Color),
}

impl From<ColorOperation> for StreamInstruction {
    fn from(value: ColorOperation) -> Self {
        match value {
            ColorOperation::StrokeColor(color @ Color::Gray(..)) => (to_vec(color), "G"),
            ColorOperation::FillColor(color @ Color::Gray(..)) => (to_vec(color), "g"),

            ColorOperation::StrokeColor(color @ Color::RGB(..)) => (to_vec(color), "RG"),
            ColorOperation::FillColor(color @ Color::RGB(..)) => (to_vec(color), "rg"),

            ColorOperation::StrokeColor(color @ Color::CMYK(..)) => (to_vec(color), "K"),
            ColorOperation::FillColor(color @ Color::CMYK(..)) => (to_vec(color), "k"),
        }
    }
}

fn to_vec(value: Color) -> Vec<PdfObj> {
    macro_rules! into {
            ($($name:ident)*) => {
                vec![
                    $(
                        ($name as f64 / 255.0).into(),
                    )*
                ]
            };
        }

    match value {
        Color::Gray(value) => into!(value),
        Color::RGB(r, g, b) => into!(r g b),
        Color::CMYK(c, m, y, k) => into!(c m y k),
    }
}