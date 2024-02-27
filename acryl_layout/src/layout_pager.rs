use acryl_core::{
    math::{Area, Pt, Vector2, VectorComponent},
    Orientation,
};
use acryl_pdf::{stream::StreamBuilder, structure::Page};

use crate::{
    layout_context::LayoutContext,
    node::{Node, NodePainter},
    painter_context::PainterContext,
};

pub struct LayoutPager {
    orientation: Orientation,
    page_size: Vector2<Pt>,
    nodes: Vec<Node>,
}

pub struct PagePainter {
    page_size: Vector2<Pt>,
    content: Vec<LayoutedPainter>
}

struct LayoutedPainter {
    area: Area<Pt>,
    painter: Option<NodePainter>,
}

impl LayoutPager {
    pub fn new(page_size: Vector2<Pt>) -> Self {
        let orientation = Orientation::Vertical;

        Self {
            orientation,
            page_size,
            nodes: Vec::new(),
        }
    }

    pub fn push(&mut self, node: Node) {
        self.nodes.push(node);
    }

    pub fn layout(self) -> Vec<PagePainter> {
        let ctx = LayoutContext {
            orientation: self.orientation.clone(),
            max_cross: self.orientation.get_cross(&self.page_size),
        };

        let main_page_size = self.orientation.get_main(&self.page_size);
        let mut position = Pt(0.0);

        let mut pages = Vec::new();
        let mut current_page = Vec::new();

        for node in self.nodes {
            let result = node.layout(&ctx);
            let size = result.size.min();

            let main_size = self.orientation.get_main(size);

            let mut next_position = position + main_size;

            if next_position >= main_page_size {
                pages.push(PagePainter {
                    page_size: self.page_size.clone(),
                    content: current_page,
                });

                current_page = Vec::new();
                position = Pt(0.0);
                next_position = main_size;
            }

            let area = Area {
                position: self.orientation.create_vector(position, Pt::ZERO),
                size: size.clone(),
            };

            let layout = LayoutedPainter {
                area,
                painter: result.painter,
            };

            current_page.push(layout);

            position = next_position;
        }

        if !current_page.is_empty() {
            pages.push(PagePainter {
                page_size: self.page_size.clone(),
                content: current_page,
            });
        }

        pages
    }
}

impl PagePainter {
    pub fn paint(self) -> Page {
        let mut page = Page::new(self.page_size);
        let mut stream_builder = StreamBuilder::new(&mut page);

        for painter in self.content {
            painter.paint(&mut stream_builder);
        }

        stream_builder.render();

        page
    }
}

impl LayoutedPainter {
    fn paint(self, stream_builder: &mut StreamBuilder<'_>) {
        let mut ctx = PainterContext {
            stream_builder,
            area: self.area,
        };

        if let Some(painter) = self.painter {
            painter.paint(&mut ctx);
        }
    }
}
