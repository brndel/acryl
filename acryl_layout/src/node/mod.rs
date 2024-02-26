mod node;
mod painter;
mod color_box_node;
mod node_result;
mod padding_node;
mod size_node;

pub use node::Node;
pub use painter::NodePainter;

use crate::layout_context::LayoutContext;
use self::node_result::NodeResult;

trait NodeLayout {
    fn layout(self, ctx: &LayoutContext) -> NodeResult;
}