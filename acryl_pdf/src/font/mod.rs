mod font;
mod glyph_info;
mod font_metrics;
mod cmap;

use std::rc::Rc;

pub use font::Font;

#[derive(Clone)]
pub struct FontRef(pub(crate) String, pub(crate) Rc<Font>);

impl FontRef {

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn font(&self) -> &Font {
        &self.1
    }
}

impl AsRef<Font> for FontRef {
    fn as_ref(&self) -> &Font {
        &self.1
    }
}