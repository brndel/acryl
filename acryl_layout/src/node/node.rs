use acryl_core::{
    math::{Pt, Vector2},
    Color,
};

use crate::{dynamic_size::DySize, layout_context::LayoutContext, padding_values::PaddingValues};

use super::painter::NodePainter;

pub enum Node {
    Color {
        color: Color,
        child: Option<Box<Self>>,
    },
    Padding {
        padding: PaddingValues<Pt>,
        child: Option<Box<Self>>,
    },
    Size {
        size: Vector2<Pt>,
        child: Option<Box<Self>>,
    },
}

pub struct NodeResult {
    pub size: DySize<Pt>,
    pub painter: Option<NodePainter>,
}

impl Node {
    pub fn layout(self, ctx: &LayoutContext) -> NodeResult {
        match self {
            Node::Color { color, child } => {
                let result = child.map(|child| child.layout(ctx));

                let (size, child) = if let Some(NodeResult { size, painter }) = result {
                    (size, painter.map(Box::new))
                } else {
                    (DySize::default(), None)
                };

                NodeResult {
                    size,
                    painter: Some(NodePainter::Color { color, child }),
                }
            }
            Node::Padding { padding, child } => {
                if let Some(child) = child {
                    let padding_vec = padding.vec();

                    let ctx = LayoutContext {
                        orientation: ctx.orientation,
                        max_cross: ctx.max_cross - ctx.orientation.get_cross(&padding_vec),
                    };

                    let NodeResult { size, painter } = child.layout(&ctx);

                    NodeResult {
                        size: size + padding.vec(),
                        painter: Some(NodePainter::Padding { padding, child: painter.map(Box::new) }),
                    }
                } else {
                    NodeResult {
                        size: DySize::Fixed(padding.vec()),
                        painter: None,
                    }
                }
            }
            Node::Size { size, child } => {
                let ctx = LayoutContext {
                    orientation: ctx.orientation,
                    max_cross: ctx.orientation.get_cross(&size),
                };

                let painter = child
                    .map(|child| child.layout(&ctx))
                    .and_then(|result| result.painter);

                NodeResult {
                    size: DySize::Fixed(size),
                    painter,
                }
            }
        }
    }
}

impl Node {
    pub fn size<T: Into<Pt>>(x: T, y: T) -> Self {
        Self::Size {
            size: Vector2::new(x.into(), y.into()),
            child: None,
        }
    }
}

impl Node {
    pub fn with_color(self, color: Color) -> Self {
        Self::Color { color, child: Some(Box::new(self)) }
    }

    pub fn with_padding(self, padding: PaddingValues<Pt>) -> Self {
        Self::Padding { padding, child: Some(Box::new(self)) }
    }
}