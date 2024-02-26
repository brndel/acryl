use acryl_core::math::{Pt, Vector2};

use crate::{dynamic_size::DySize, layout_context::LayoutContext};

use super::{node_result::NodeResult, Node, NodeLayout};


pub struct SizeNode {
    pub size: Vector2<Pt>,
    pub child: Option<Box<Node>>
}

impl NodeLayout for SizeNode {
    fn layout(self, ctx: &LayoutContext) -> NodeResult {
        let ctx = LayoutContext {
            orientation: ctx.orientation,
            max_cross: ctx.orientation.get_cross(&self.size),
        };

        let painter = self.child
            .map(|child| child.layout(&ctx))
            .and_then(|result| result.painter);

        NodeResult {
            size: DySize::Fixed(self.size),
            painter,
        }
    }
}