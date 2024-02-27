mod node;
mod painter;
mod color_box;
mod node_result;
mod padding;
mod size_node;

pub use node::Node;
pub use painter::NodePainter;

use crate::{layout_context::LayoutContext, painter_context::PainterContext};
use self::node_result::NodeResult;

trait NodeLayout {
    fn layout(self, ctx: &LayoutContext) -> NodeResult;
}

trait NodePaint {
    fn paint(self, ctx: &mut PainterContext);
}