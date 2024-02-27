use crate::painter_context::PainterContext;

use super::{color_box::ColorBoxPainter, padding::PaddingPainter, NodePaint};

pub enum NodePainter {
    ColorBox(ColorBoxPainter),
    Padding(PaddingPainter),
}

impl NodePainter {
    pub fn paint(self, ctx: &mut PainterContext) {
        match self {
            NodePainter::ColorBox(painter) => painter.paint(ctx),
            NodePainter::Padding(painter) => painter.paint(ctx),
        }
    }
}
