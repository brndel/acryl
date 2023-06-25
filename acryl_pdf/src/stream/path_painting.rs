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

impl Into<StreamInstruction> for PathPainting {
    fn into(self) -> StreamInstruction {
        let operator = match self {
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