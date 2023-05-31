use crate::render::PdfObj;

use super::Vector2;

#[derive(Clone)]
pub struct Area {
    pub position: Vector2,
    pub size: Vector2,
}

impl Into<PdfObj> for Area {
    fn into(self) -> PdfObj {
        vec![
            self.position.x,
            self.position.y,
            self.position.x + self.size.x,
            self.position.y + self.size.y,
        ]
        .into()
    }
}

impl Area {
    pub fn from_size(size: Vector2) -> Self {
        Self {
            position: Vector2::default(),
            size,
        }
    }
}
