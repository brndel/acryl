use acryl_core::{
    math::{Pt, Vector2},
    Color,
};

use crate::{layout_context::LayoutContext, padding_values::PaddingValues};

use super::{color_box_node::ColorBoxNode, node_result::NodeResult, padding_node::PaddingNode, size_node::SizeNode, NodeLayout};

pub enum Node {
    ColorBox(ColorBoxNode),
    Padding(PaddingNode),
    Size(SizeNode),
}

impl Node {
    pub fn layout(self, ctx: &LayoutContext) -> NodeResult {
        match self {
            Node::ColorBox(node) => {
                node.layout(ctx)
            }
            Node::Padding(node) => {
                node.layout(ctx)
            }
            Node::Size(node) => {
                node.layout(ctx)
            }
        }
    }
}

impl Node {
    pub fn size<T: Into<Pt>>(x: T, y: T) -> Self {
        Self::Size(SizeNode {
            size: Vector2::new(x.into(), y.into()),
            child: None,
        })
    }
}

impl Node {
    pub fn with_color(self, color: Color) -> Self {
        Self::ColorBox(ColorBoxNode { color, child: Some(Box::new(self)) })
    }

    pub fn with_padding(self, padding: PaddingValues<Pt>) -> Self {
        Self::Padding(PaddingNode { padding, child: Some(Box::new(self)) })
    }
}