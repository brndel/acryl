use acryl_core::math::{Pt, Matrix};

use super::StreamInstruction;

pub enum GraphicsState {
    SaveState,
    RestoreState,
    TransformMatrix(Matrix<Pt, 2, 3>),
    LineWidth(Pt),
    LineCap(LineCap),
    LineJoin(LineJoin),
    MiterLimit(f64),
    DashPattern(Vec<Pt>, u32),
    // Intent(intent)
    // Flatness(flatness)
    // DictNum(dict_name)
}

#[repr(u8)]
pub enum LineCap {
    Butt = 0,
    Round,
    Sqare,
}

#[repr(u8)]
pub enum LineJoin {
    Miter = 0,
    Round,
    Bevel,
}

impl From<GraphicsState> for StreamInstruction {
    fn from(value: GraphicsState) -> Self {
        match value {
            GraphicsState::SaveState => (vec![], "q"),
            GraphicsState::RestoreState => (vec![], "Q"),
            GraphicsState::TransformMatrix(matrix) => {
                let points: Vec<Pt> = matrix.into();
                (points.into_iter().map(|p| p.into()).collect(), "cm")
            }
            GraphicsState::LineWidth(width) => (vec![width.into()], "w"),
            GraphicsState::LineCap(cap) => (vec![(cap as u8).into()], "J"),
            GraphicsState::LineJoin(join) => (vec![(join as u8).into()], "j"),
            GraphicsState::MiterLimit(limit) => (vec![limit.into()], "M"),
            GraphicsState::DashPattern(array, phase) => (vec![array.into(), phase.into()], "d"),
            // Intent(intent) => "ri"
            // Flatness(flatness) => "i"
            // DictNum(dict_name) => "gs"
        }
    }
}
