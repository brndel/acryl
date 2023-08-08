use acryl_core::{VectorComponent, Vector2, Area};


pub trait CoordinateTransformer<T> {
    fn transform(&self, value: T) -> T;
}

impl<T: VectorComponent> CoordinateTransformer<Vector2<T>> for Area<T> {
    fn transform(&self, value: Vector2<T>) -> Vector2<T> {
        Vector2 {
            x: value.x - self.left(),
            y: self.bottom() - (value.y - self.top()),
        }
    }
}

impl<T: VectorComponent> CoordinateTransformer<Area<T>> for Area<T> {
    fn transform(&self, value: Area<T>) -> Area<T> {
        let mut position = self.transform(value.bottom_left());
        position.y -= value.size.y;
        Area {
            position,
            size: value.size,
        }
    }
}
