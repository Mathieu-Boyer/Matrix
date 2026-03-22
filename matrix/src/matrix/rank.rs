use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K : Field, const N : usize, const M : usize> Matrix<K, N , M> {
    pub fn rank(&self) -> usize{
        let row_echelon_mat = self.row_echelon();
        let mut rank = 0;
        
        for i in 0..N {
            for j in 0..M{
                if row_echelon_mat.data[i][j] != K::default(){
                    rank += 1;
                    break;
                }
            }
        }
        rank
    }
}