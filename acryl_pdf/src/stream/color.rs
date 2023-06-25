use super::StreamInstruction;

pub enum Color {
    Gray(f64),
    RGB(f64, f64, f64),
    CMYK(f64, f64, f64, f64),
}

pub enum ColorOperation {
    StrokeColor(Color),
    FillColor(Color),
}

impl Into<StreamInstruction> for ColorOperation {
    fn into(self) -> StreamInstruction {
        match self {
            ColorOperation::StrokeColor(Color::Gray(value)) => (vec![value.into()], "G"),
            ColorOperation::FillColor(Color::Gray(value)) => (vec![value.into()], "g"),

            ColorOperation::StrokeColor(Color::RGB(r, g, b)) => {
                (vec![r.into(), g.into(), b.into()], "RG")
            }
            ColorOperation::FillColor(Color::RGB(r, g, b)) => {
                (vec![r.into(), g.into(), b.into()], "rg")
            }

            ColorOperation::StrokeColor(Color::CMYK(c, m, y, k)) => {
                (vec![c.into(), m.into(), y.into(), k.into()], "K")
            }
            ColorOperation::FillColor(Color::CMYK(c, m, y, k)) => {
                (vec![c.into(), m.into(), y.into(), k.into()], "k")
            }
        }
    }
}
