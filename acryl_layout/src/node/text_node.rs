use acryl_pdf::{font::{Font, WordLayout}, resource::resource_manager::ResourceRef};

use crate::{dynamic_size::DySize, layout_context::LayoutContext, painter_context::PainterContext};

use super::{node_result::NodeResult, Node, NodeLayout, NodePaint, NodePainter};


pub struct TextNode {
    pub text: String,
    pub font_size: f64,
    pub font: ResourceRef<Font>,
}

impl From<TextNode> for Node {
    fn from(value: TextNode) -> Self {
        Self::Text(value)
    }
}

impl NodeLayout for TextNode {
    fn layout(self, ctx: &LayoutContext) -> NodeResult {
        let font = self.font.data();

        let mut words = Vec::new();

        for word in self.text.split_whitespace() {
            let layout = font.layout(word);
            words.push(layout);
        }

        let size = DySize::default();

        NodeResult::new(size, TextPainter {
            words,
            font: self.font,
            font_size: self.font_size
        })
    }
}

pub struct TextPainter {
    words: Vec<WordLayout>,
    font_size: f64,
    font: ResourceRef<Font>,
}

impl From<TextPainter> for NodePainter {
    fn from(value: TextPainter) -> Self {
        Self::Text(value)
    }
}

impl NodePaint for TextPainter {
    fn paint(self, ctx: &mut PainterContext) {
        println!("painting Text at {:?}", ctx.area);
        let mut text_builder = ctx.stream_builder.text(&self.font, self.font_size);

        // text_builder.set_position(ctx.area.position.clone());

        for word in self.words {
            text_builder.draw_word(word)
        }

        drop(text_builder);
    }
}