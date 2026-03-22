use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl <K : Field, const N : usize , const M : usize> Matrix<K, N , M> {

    pub fn transpose(&self) -> Matrix<K, M , N> {
        let mut new_matrix: Matrix<K, M, N> =  Matrix::new([[K::default() ; N] ; M]);

        for i in 0..M {
            for j in 0..N {
                new_matrix.data[i][j] = self.data[j][i];
            }
        }
        new_matrix
    }
}