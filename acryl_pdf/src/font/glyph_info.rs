use crate::{Vector2, unit::Pt};


#[derive(Debug)]
pub struct GlyphInfo<'a> {
    pub id: u16,
    pub name: &'a str,
    pub advance: Vector2<Pt>
}