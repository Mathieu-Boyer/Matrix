use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K : Field , const N : usize> Matrix<K, N, N> {
    pub fn trace(&self) -> K{
        let mut sum = K::default();
        for i in 0..N {
            sum = sum + self.data[i][i];
        }
        return sum
    }
}