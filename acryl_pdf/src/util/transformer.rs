
pub trait CoordinateTransformer<T> {
    fn transform(&self, value: T) -> T;
}