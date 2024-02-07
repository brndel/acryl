use crate::math::vector::VectorComponent;


pub struct Matrix<T: VectorComponent, const Y: usize, const X: usize> {
    elements: [[T; X]; Y]
}

impl<T: VectorComponent, const Y: usize, const X: usize> Matrix<T, Y, X> {
    pub fn get(&self, x: usize, y: usize) -> Option<T> {
        if x < X && y < Y {
            Some(self.elements[y][x])
        } else {
            None
        }
    }
}

impl<T: VectorComponent, const Y: usize, const X: usize> Into<Vec<T>> for Matrix<T, Y, X> {
    fn into(self) -> Vec<T> {
        let mut v = Vec::new();

        for y in 0..Y {
            for x in 0..X {
                v.push(self.elements[y][x])
            }
        }

        v
    }
}