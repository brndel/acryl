use acryl_core::Color;
use acryl_pdf::stream::{FillPaintArgs, FillRule, StrokePaintArgs};

use crate::{layout_context::LayoutContext, painter_context::PainterContext};

use super::{node_result::NodeResult, Node, NodeLayout, NodePaint, NodePainter};

pub struct ColorBoxNode {
    pub fill: Option<FillPaintArgs>,
    pub stroke: Option<StrokePaintArgs>,
    pub child: Option<Box<Node>>,
}

impl ColorBoxNode {
    pub fn color<T: Into<Node>>(color: Color, child: Option<Node>) -> Self {
        Self {
            fill: Some(FillPaintArgs {
                color,
                fill_rule: FillRule::NonzeroWinding,
            }),
            stroke: None,
            child: child.map(|child| Box::new(child.into())),
        }
    }
}

impl From<ColorBoxNode> for Node {
    fn from(value: ColorBoxNode) -> Self {
        Self::ColorBox(value)
    }
}

impl NodeLayout for ColorBoxNode {
    fn layout(self, ctx: &LayoutContext) -> NodeResult {
        if let Some(child) = self.child {
            let NodeResult { size, painter } = child.layout(ctx);

            NodeResult::new(size, ColorBoxPainter::new(self.fill, self.stroke, painter))
        } else {
            NodeResult::default()
        }
    }
}

pub struct ColorBoxPainter {
    fill: Option<FillPaintArgs>,
    stroke: Option<StrokePaintArgs>,
    child: Option<Box<NodePainter>>,
}

impl ColorBoxPainter {
    fn new<T: Into<NodePainter>>(
        fill: Option<FillPaintArgs>,
        stroke: Option<StrokePaintArgs>,
        child: Option<T>,
    ) -> Self {
        Self {
            fill,
            stroke,
            child: child.map(|child| Box::new(child.into())),
        }
    }
}

impl From<ColorBoxPainter> for NodePainter {
    fn from(value: ColorBoxPainter) -> Self {
        Self::ColorBox(value)
    }
}

impl NodePaint for ColorBoxPainter {
    fn paint(self, ctx: &mut PainterContext) {
        let mut path = ctx.stream_builder.path();

        path.rect(ctx.area.clone());

        path.paint(self.fill, self.stroke);

        if let Some(child) = self.child {
            child.paint(ctx);
        }
    }
}
