use acryl_core::{Vector2, unit::Pt, Area};
use acryl_pdf::stream::Streambuilder;

pub mod linear_layout;
pub mod color_box;
pub mod padding;
pub mod text;
pub mod size_box;


pub trait LayoutElement {
    fn get_min_size(&self, max_size: Vector2<Pt>) -> Vector2<Pt>;
    fn render(&self, area: Area<Pt>, builder: &mut Streambuilder);
}

pub type LayoutBox = Box<dyn LayoutElement>;