use acryl_core::{unit::Pt, Area, Vector2};

use super::StreamInstruction;

pub enum PathConstruction {
    Move(Vector2<Pt>),
    LineTo(Vector2<Pt>),
    CubicBezier {
        p1: Vector2<Pt>,
        p2: Vector2<Pt>,
        p3: Vector2<Pt>,
    },
    CubicBezierAutoStart {
        p2: Vector2<Pt>,
        p3: Vector2<Pt>,
    },
    CubicBezierAutoEnd {
        p1: Vector2<Pt>,
        p2: Vector2<Pt>,
    },
    Close,
    Rect(Area<Pt>),
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
                let pos = area.bottom_left();
                (
                    vec![
                        pos.x.into(),
                        pos.y.into(),
                        area.size.x.into(),
                        area.size.y.into(),
                    ],
                    "re",
                )
            }
        }
    }
}
