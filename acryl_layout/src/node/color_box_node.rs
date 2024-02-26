use acryl_core::Color;

use crate::{dynamic_size::DySize, layout_context::LayoutContext};

use super::{node_result::NodeResult, Node, NodeLayout, NodePainter};


pub struct ColorBoxNode {
    pub color: Color,
    pub child: Option<Box<Node>>
}

impl NodeLayout for ColorBoxNode {
    fn layout(self, ctx: &LayoutContext) -> NodeResult {
        if let Some(child) = self.child {
            
            let NodeResult { size, painter } = child.layout(ctx);
    
            NodeResult {
                size,
                painter: Some(NodePainter::Color { color: self.color, child: painter.map(Box::new) }),
            }
        } else {
            NodeResult {
                size: DySize::default(),
                painter: None
            }
        }
    }
}