use acryl_core::math::Pt;

use crate::dynamic_size::DySize;

use super::NodePainter;

pub struct NodeResult {
    pub size: DySize<Pt>,
    pub painter: Option<NodePainter>,
}
