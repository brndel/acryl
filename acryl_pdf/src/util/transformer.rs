use acryl_core::math::{AcrylCoords, Area, PdfCoords, Vector2, VectorComponent};


pub trait CoordinateTransformer<T, R> {
    fn transform(&self, value: T) -> R;
}

impl<T: VectorComponent> CoordinateTransformer<Vector2<T, AcrylCoords>, Vector2<T, PdfCoords>> for Area<T, AcrylCoords> {
    fn transform(&self, value: Vector2<T, AcrylCoords>) -> Vector2<T, PdfCoords> {
        Vector2::new(
            value.x - self.left(),
            self.bottom() - (value.y - self.top()),
        )
    }
}

impl<T: VectorComponent> CoordinateTransformer<Area<T, AcrylCoords>, Area<T, PdfCoords>> for Area<T, AcrylCoords> {
    fn transform(&self, value: Area<T, AcrylCoords>) -> Area<T, PdfCoords> {
        let mut position = self.transform(value.bottom_left());
        position.y -= value.size.y;
        Area {
            position,
            size: value.size.with_coords::<PdfCoords>(),
        }
    }
}
