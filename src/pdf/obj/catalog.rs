use super::{Id, Page};

pub struct Catalog {
  id: Id,
  pages: Pages,
}


pub struct Pages {
  id: Id,
  kids: Vec<Page>,
}
