use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K : Field, const N : usize> Matrix<K, N, N> {
    pub fn inverse(&self) -> Result<Matrix<K, N, N> , &str>{

        let mut right = Matrix::identity::<N, N>();
        let comparison_mat = right.clone();



        let mat_after_ref = self.row_echelon_augmented(&mut right);
        for i in 0..N {
            for j in 0..N {
                if mat_after_ref.data[i][j] != comparison_mat.data[i][j] {
                    return Err("The matrix is Singular");
                }
            }
        }

        Ok(right)
    }
}