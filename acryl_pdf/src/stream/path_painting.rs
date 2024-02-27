use super::StreamInstruction;

/// PDF Book 8.5.3 [Path-Painting Operators][https://opensource.adobe.com/dc-acrobat-sdk-docs/standards/pdfstandards/pdf/PDF32000_2008.pdf#G7.3799841]
pub enum PathPainting {
    /// Stroke the path
    Stroke,
    /// Close and stroke the path
    CloseStroke,
    /// Fill the path
    Fill(FillRule),
    /// Fill and then stroke the path
    FillStroke(FillRule),
    /// Close, fill and then stroke the path
    CloseFillStroke(FillRule),
    /// End the path without filling or stroking it
    End,
}

/// PDF Book 8.5.3.3 [Filling][https://opensource.adobe.com/dc-acrobat-sdk-docs/standards/pdfstandards/pdf/PDF32000_2008.pdf#G7.3901540]
#[derive(Debug, Clone, Copy)]
pub enum FillRule {
    NonzeroWinding,
    EvenOdd,
}

impl PathPainting {
    pub fn new(fill: Option<FillRule>, close_stroke: Option<bool>) -> Self {
        match (fill, close_stroke) {
            (None, None) => Self::End,
            (None, Some(close_stroke)) => {
                if close_stroke {
                    Self::CloseStroke
                } else {
                    Self::Stroke
                }
            }
            (Some(fill_rule), None) => Self::Fill(fill_rule),
            (Some(fill_rule), Some(close_stroke)) => {
                if close_stroke {
                    Self::CloseFillStroke(fill_rule)
                } else {
                    Self::FillStroke(fill_rule)
                }
            }
        }
    }
}

impl From<PathPainting> for StreamInstruction {
    fn from(value: PathPainting) -> Self {
        let operator = match value {
            PathPainting::Stroke => "S",
            PathPainting::CloseStroke => "s",
            PathPainting::Fill(FillRule::NonzeroWinding) => "f",
            PathPainting::Fill(FillRule::EvenOdd) => "f*",
            PathPainting::FillStroke(FillRule::NonzeroWinding) => "B",
            PathPainting::FillStroke(FillRule::EvenOdd) => "B*",
            PathPainting::CloseFillStroke(FillRule::NonzeroWinding) => "b",
            PathPainting::CloseFillStroke(FillRule::EvenOdd) => "b*",
            PathPainting::End => "n",
        };

        (vec![], operator)
    }
}
