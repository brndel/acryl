use acryl_core::math::{Area, Unit, Vector2, VectorComponent};

#[derive(Debug, Clone)]
pub struct PaddingValues<U: Unit> {
    pub top: U,
    pub bottom: U,
    pub left: U,
    pub right: U,
}

impl<U: Unit> PaddingValues<U> {
    pub fn all(value: U) -> Self {
        Self {
            top: value,
            bottom: value,
            left: value,
            right: value,
        }
    }

    pub fn vert_hor(vertical: U, horizontal: U) -> Self {
        Self {
            top: vertical,
            bottom: vertical,
            left: horizontal,
            right: horizontal,
        }
    }

    pub fn vertical(&self) -> U {
        self.top + self.bottom
    }

    pub fn horizontal(&self) -> U {
        self.left + self.right
    }

    pub fn vec(&self) -> Vector2<U>
    where
        U: VectorComponent,
    {
        Vector2::new(self.horizontal(), self.vertical())
    }

    pub fn apply(&self, area: &Area<U>) -> Area<U>
    where
        U: VectorComponent,
    {
        let offset = Vector2::new(self.left, self.top);
        Area {
            position: area.position.clone() + offset.clone(),
            size: area.size.clone() - offset - Vector2::new(self.right, self.bottom),
        }
    }
}
