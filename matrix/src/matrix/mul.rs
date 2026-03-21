use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;
use crate::vector::core::Vector;

impl<K: Field, const N : usize, const M : usize> Matrix<K, N, M> {

    pub fn mul_vec(&self, other: &Vector<K, M>) -> Vector<K, M> {
        let mut result = Vector::new([K::default(); M]);
        for i in 0..N {
            for j in 0..M {
                result.data[i] = result.data[i] +  self.data[i][j] * other.data[j];
            }
        }
        result
    }
    
    pub fn mul_mat<const P : usize>(&self, other: &Matrix<K, M, P>) -> Matrix<K, N, P> {
        let mut result = Matrix::new([[K::default(); P]; N]);
        for i in 0..N {
            for j in 0..P {
                for k in 0..M {
                    result.data[i][j] = result.data[i][j] + self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }
}