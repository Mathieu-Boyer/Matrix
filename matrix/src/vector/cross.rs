use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;
impl<K : Field, const N: usize>  Vector<K, N> {
    pub fn cross(&self, other: &Vector<K, N>) -> Vector<K, N> {
        let mut result =  Vector::new([K::default(); N]);

        result.data[0] = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        result.data[1] = self.data[2] * other.data[0] - self.data[0] * other.data[2];
        result.data[2] = self.data[0] * other.data[1] - self.data[1] * other.data[0];

        result
    }
}