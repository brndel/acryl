use acryl_core::math::Pt;

use crate::{dynamic_size::DySize, layout_context::LayoutContext, padding_values::PaddingValues};

use super::{node_result::NodeResult, Node, NodeLayout, NodePainter};


pub struct PaddingNode {
    pub padding: PaddingValues<Pt>,
    pub child: Option<Box<Node>>
}

impl NodeLayout for PaddingNode {
    fn layout(self, ctx: &LayoutContext) -> NodeResult {
        if let Some(child) = self.child {
            let padding_vec = self.padding.vec();

            let ctx = LayoutContext {
                orientation: ctx.orientation,
                max_cross: ctx.max_cross - ctx.orientation.get_cross(&padding_vec),
            };

            let NodeResult { size, painter } = child.layout(&ctx);

            NodeResult {
                size: size + self.padding.vec(),
                painter: Some(NodePainter::Padding { padding: self.padding, child: painter.map(Box::new) }),
            }
        } else {
            NodeResult {
                size: DySize::Fixed(self.padding.vec()),
                painter: None,
            }
        }
    }
}