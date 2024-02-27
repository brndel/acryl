use acryl_core::math::{Area, PdfCoords, Pt, Vector2};

use super::StreamInstruction;

/// PDF Book 8.5.2 [Path Construction Operators][https://opensource.adobe.com/dc-acrobat-sdk-docs/standards/pdfstandards/pdf/PDF32000_2008.pdf#G7.3799534]
pub enum PathConstruction {
    /// Begin a new subpath by moving the current point to the given point
    MoveTo(Vector2<Pt, PdfCoords>),
    /// Append a straight line segment from the current point to the given point
    LineTo(Vector2<Pt, PdfCoords>),
    /// Append a cubic Bézier to the current path.
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
    /// Close the current subpath by appending a straight line segment from the current point to the starting point of the subpath. If the current subpath is already closed, do nothing
    Close,
    /// Append a rectangle to the current path as a complete subpath
    Rect(Area<Pt, PdfCoords>),
}

impl From<PathConstruction> for StreamInstruction {
    fn from(value: PathConstruction) -> Self {
        macro_rules! into {
            ($($name:expr)*) => {
                vec![
                    $(
                        $name.x.into(),
                        $name.y.into(),
                    )*
                ]
            };
        }
        match value {
            PathConstruction::MoveTo(pos) => (into!(pos), "m"),
            PathConstruction::LineTo(pos) => (into!(pos), "l"),
            PathConstruction::CubicBezier { p1, p2, p3 } => (into!(p1 p2 p3), "c"),
            PathConstruction::CubicBezierAutoStart { p2, p3 } => (into!(p2 p3), "v"),
            PathConstruction::CubicBezierAutoEnd { p1, p2 } => (into!(p1 p2), "y"),
            PathConstruction::Close => (vec![], "h"),
            PathConstruction::Rect(area) => {
                (
                    into!(area.position area.size),
                    "re",
                )
            }
        }
    }
}
