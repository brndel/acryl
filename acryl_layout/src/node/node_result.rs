use acryl_core::math::Pt;

use crate::dynamic_size::DySize;

use super::NodePainter;

#[derive(Default)]
pub struct NodeResult {
    pub size: DySize<Pt>,
    pub painter: Option<NodePainter>,
}

impl NodeResult {
    pub fn new<T: Into<NodePainter>>(size: DySize<Pt>, painter: T) -> Self {
        Self {
            size,
            painter: Some(painter.into()),
        }
    }

    pub fn new_opt<T: Into<NodePainter>>(size: DySize<Pt>, painter: Option<T>) -> Self {
        Self {
            size,
            painter: painter.map(Into::into),
        }
    }
}
