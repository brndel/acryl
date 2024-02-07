use acryl_core::math::{Area, PdfCoords, Pt, Vector2};

use super::StreamInstruction;

pub enum PathConstruction {
    Move(Vector2<Pt, PdfCoords>),
    LineTo(Vector2<Pt, PdfCoords>),
    CubicBezier {
        p1: Vector2<Pt, PdfCoords>,
        p2: Vector2<Pt, PdfCoords>,
        p3: Vector2<Pt, PdfCoords>,
    },
    CubicBezierAutoStart {
        p2: Vector2<Pt, PdfCoords>,
        p3: Vector2<Pt, PdfCoords>,
    },
    CubicBezierAutoEnd {
        p1: Vector2<Pt, PdfCoords>,
        p2: Vector2<Pt, PdfCoords>,
    },
    Close,
    Rect(Area<Pt, PdfCoords>),
}

impl From<PathConstruction> for StreamInstruction {
    fn from(value: PathConstruction) -> Self {
        macro_rules! into {
            ($($name:ident)*) => {
                vec![
                    $(
                        $name.x.into(),
                        $name.y.into(),
                    )*
                ]
            };
        }
        match value {
            PathConstruction::Move(pos) => (into!(pos), "m"),
            PathConstruction::LineTo(pos) => (into!(pos), "l"),
            PathConstruction::CubicBezier { p1, p2, p3 } => (into!(p1 p2 p3), "c"),
            PathConstruction::CubicBezierAutoStart { p2, p3 } => (into!(p2 p3), "v"),
            PathConstruction::CubicBezierAutoEnd { p1, p2 } => (into!(p1 p2), "y"),
            PathConstruction::Close => (vec![], "h"),
            PathConstruction::Rect(area) => {
                let size = &area.size;
                let pos = area.bottom_left();
                (
                    into!(pos size),
                    "re",
                )
            }
        }
    }
}
