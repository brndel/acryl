use crate::painter_context::PainterContext;

use super::{color_box::ColorBoxPainter, padding::PaddingPainter, text_node::TextPainter, NodePaint};

pub enum NodePainter {
    ColorBox(ColorBoxPainter),
    Padding(PaddingPainter),
    Text(TextPainter),
}

impl NodePainter {
    pub fn paint(self, ctx: &mut PainterContext) {
        match self {
            NodePainter::ColorBox(painter) => painter.paint(ctx),
            NodePainter::Padding(painter) => painter.paint(ctx),
            NodePainter::Text(painter) => painter.paint(ctx),
        }
    }
}
