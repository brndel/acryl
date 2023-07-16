mod font;
mod glyph_info;
mod font_metrics;
mod cmap;

pub use font::Font;

#[derive(Clone)]
pub struct FontRef(pub(crate) String);

impl AsRef<String> for FontRef {
    fn as_ref(&self) -> &String {
        &self.0
    }
}