use acryl_core::math::Pt;

use crate::{
    dynamic_size::DySize, layout_context::LayoutContext, padding_values::PaddingValues,
    painter_context::PainterContext,
};

use super::{node_result::NodeResult, Node, NodeLayout, NodePaint, NodePainter};

pub struct PaddingNode {
    pub padding: PaddingValues<Pt>,
    pub child: Option<Box<Node>>,
}

impl From<PaddingNode> for Node {
    fn from(value: PaddingNode) -> Self {
        Self::Padding(value)
    }
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


            NodeResult::new_opt(size, PaddingPainter::new_opt(self.padding, painter))
        } else {
            NodeResult {
                size: DySize::Fixed(self.padding.vec()),
                painter: None,
            }
        }
    }
}

pub struct PaddingPainter {
    padding: PaddingValues<Pt>,
    child: Box<NodePainter>,
}

impl PaddingPainter {
    pub fn new<T: Into<NodePainter>>(padding: PaddingValues<Pt>, child: T) -> Self {
        Self {
            padding,
            child: Box::new(child.into()),
        }
    }

    pub fn new_opt<T: Into<NodePainter>>(
        padding: PaddingValues<Pt>,
        child: Option<T>,
    ) -> Option<Self> {
        if let Some(child) = child {
            Some(Self::new(padding, child))
        } else {
            None
        }
    }
}

impl From<PaddingPainter> for NodePainter {
    fn from(value: PaddingPainter) -> Self {
        Self::Padding(value)
    }
}

impl NodePaint for PaddingPainter {
    fn paint(self, ctx: &mut PainterContext) {
        let mut ctx = PainterContext {
            stream_builder: ctx.stream_builder,
            area: self.padding.apply(&ctx.area),
        };

        self.child.paint(&mut ctx);
    }
}
