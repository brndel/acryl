use acryl_core::Color;

use super::StreamInstruction;

pub enum ColorOperation {
    StrokeColor(Color),
    FillColor(Color),
}

impl From<ColorOperation> for StreamInstruction {
    fn from(value: ColorOperation) -> Self {
        macro_rules! to_vec {
            ($($name:ident)*) => {
                vec![
                    $(
                        ($name as f64 / 255.0).into(),
                    )*
                ]
            };
        }

        match value {
            ColorOperation::StrokeColor(Color::Gray(value)) => (to_vec!(value), "G"),
            ColorOperation::FillColor(Color::Gray(value)) => (to_vec!(value), "g"),

            ColorOperation::StrokeColor(Color::RGB(r, g, b)) => (to_vec!(r g b), "RG"),
            ColorOperation::FillColor(Color::RGB(r, g, b)) => (to_vec!(r g b), "rg"),

            ColorOperation::StrokeColor(Color::CMYK(c, m, y, k)) => (to_vec!(c m y k), "K"),
            ColorOperation::FillColor(Color::CMYK(c, m, y, k)) => (to_vec!(c m y k), "k"),
        }
    }
}
