mod external;
mod glyph_info;
mod face_metrics;

pub use external::ExternalFont;

#[derive(Clone)]
pub struct FontRef(pub(crate) String);

impl AsRef<String> for FontRef {
    fn as_ref(&self) -> &String {
        &self.0
    }
}