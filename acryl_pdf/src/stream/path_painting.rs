use super::StreamInstruction;



pub enum PathPainting {
    Stroke,
    CloseStroke,
    Fill(FillRule),
    FillStroke(FillRule),
    CloseFillStroke(FillRule),
    End
}

pub enum FillRule {
    NonzeroWinding,
    EvenOdd
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