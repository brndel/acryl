use crate::{unit::Pt, util::Area, Vector2};

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

impl Into<StreamInstruction> for PathConstruction {
    fn into(self) -> StreamInstruction {
        match self {
            PathConstruction::Move(pos) => (vec![pos.x.into(), pos.y.into()], "m"),
            PathConstruction::LineTo(pos) => (vec![pos.x.into(), pos.y.into()], "l"),
            PathConstruction::CubicBezier { p1, p2, p3 } => (
                vec![
                    p1.x.into(),
                    p2.y.into(),
                    p2.x.into(),
                    p2.y.into(),
                    p3.x.into(),
                    p3.y.into(),
                ],
                "c",
            ),
            PathConstruction::CubicBezierAutoStart { p2, p3 } => (
                vec![p2.x.into(), p2.y.into(), p3.x.into(), p3.y.into()],
                "v",
            ),
            PathConstruction::CubicBezierAutoEnd { p1, p2 } => (
                vec![p1.x.into(), p1.y.into(), p2.x.into(), p2.y.into()],
                "y",
            ),
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
