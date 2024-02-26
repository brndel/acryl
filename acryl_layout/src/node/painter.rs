use acryl_core::{math::Pt, Color};

use crate::{padding_values::PaddingValues, painter_context::PainterContext};


pub enum NodePainter {
    Color {
        color: Color,
        child: Option<Box<Self>>,
    },
    Padding {
        padding: PaddingValues<Pt>,
        child: Option<Box<Self>>,
    },
}

impl NodePainter {
    pub fn paint(self, ctx: &mut PainterContext) {
        match self {
            NodePainter::Color { color, child } => {
                ctx.stream_builder.draw_rect(ctx.area.clone(), color);
                
                if let Some(child) = child {
                    child.paint(ctx);
                }
            },
            NodePainter::Padding { padding, child } => {
                if let Some(child) = child {
                    let mut ctx = PainterContext {
                        stream_builder: ctx.stream_builder,
                        area: padding.apply(&ctx.area),
                    };

                    child.paint(&mut ctx);
                }
            },
        }
    }
}