use acryl_core::math::{Area, Pt};

use crate::{
    font::Font, resource::resource_manager::ResourceRef, stream::{
        Stream, StreamInstruction,
    }, structure::Page, util::CoordinateTransformer
};

use super::{path_builder::PathBuilder, text_builder::TextBuilder};

pub struct StreamBuilder<'page> {
    page: &'page mut Page,
    instructions: Vec<StreamInstruction>,
}

impl<'page> StreamBuilder<'page> {
    pub fn new(page: &'page mut Page) -> Self {
        Self {
            page,
            instructions: Vec::new(),
        }
    }

    pub fn get_area(&self) -> &Area<Pt> {
        &self.page.area()
    }

    pub fn render(self) {
        self.page.add_stream(Stream::new(self.instructions))
    }

    pub(super) fn push<T: Into<StreamInstruction>>(&mut self, instr: T) {
        self.instructions.push(instr.into())
    }

    pub fn text<'builder, 'font>(
        &'builder mut self,
        font_ref: &ResourceRef<Font>,
        size: f64,
    ) -> TextBuilder<'builder, 'page> {
        TextBuilder::new(self, font_ref, size)
    }

    pub fn path<'builder>(
        &'builder mut self
    ) -> PathBuilder<'builder, 'page> {
        PathBuilder::new(self)
    }
}

impl<'page> StreamBuilder<'page> {
    pub(super) fn transform<T, R>(&self, value: T) -> R
    where
        Page: CoordinateTransformer<T, R>,
    {
        self.page.transform(value)
    }
}
