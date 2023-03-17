use super::{Stream, Rect, Id};

pub struct Page {
  pub id: Id,
  contents: Vec<Stream>,
  size: Rect,
}