use super::StreamInstruction;

#[derive(Clone)]
pub enum Color {
    Gray(u8),
    RGB(u8, u8, u8),
    CMYK(u8, u8, u8, u8),
}

pub enum ColorOperation {
    StrokeColor(Color),
    FillColor(Color),
}

impl Into<StreamInstruction> for ColorOperation {
    fn into(self) -> StreamInstruction {
        match self {
            ColorOperation::StrokeColor(Color::Gray(value)) => {
                (vec![(value as f64 / 255.0).into()], "G")
            }
            ColorOperation::FillColor(Color::Gray(value)) => {
                (vec![(value as f64 / 255.0).into()], "g")
            }

            ColorOperation::StrokeColor(Color::RGB(r, g, b)) => (
                vec![
                    (r as f64 / 255.0).into(),
                    (g as f64 / 255.0).into(),
                    (b as f64 / 255.0).into(),
                ],
                "RG",
            ),
            ColorOperation::FillColor(Color::RGB(r, g, b)) => (
                vec![
                    (r as f64 / 255.0).into(),
                    (g as f64 / 255.0).into(),
                    (b as f64 / 255.0).into(),
                ],
                "rg",
            ),

            ColorOperation::StrokeColor(Color::CMYK(c, m, y, k)) => (
                vec![
                    (c as f64 / 255.0).into(),
                    (m as f64 / 255.0).into(),
                    (y as f64 / 255.0).into(),
                    (k as f64 / 255.0).into(),
                ],
                "K",
            ),
            ColorOperation::FillColor(Color::CMYK(c, m, y, k)) => (
                vec![
                    (c as f64 / 255.0).into(),
                    (m as f64 / 255.0).into(),
                    (y as f64 / 255.0).into(),
                    (k as f64 / 255.0).into(),
                ],
                "k",
            ),
        }
    }
}

impl Color {
    pub fn rgb_from_hex(value: u64) -> Self {
        let b = ((value & 0x000000ff) >> 0) as u8;
        let g = ((value & 0x0000ff00) >> 8) as u8;
        let r = ((value & 0x00ff0000) >> 16) as u8;

        Color::RGB(r, g, b)
    }
}
