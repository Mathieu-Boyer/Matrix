use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K: Field, const N : usize, const M : usize> Matrix<K, N , M> {
    pub fn row_echelon(&self) -> Matrix<K, N, M> {
        let mut new_matrix = self.clone();
        let mut current_row = 0;

        for col in 0..M {
            let mut pivot_row = None;
            for row in current_row..N {
                if new_matrix.data[row][col] != K::default() {
                    pivot_row = Some(row);
                    break;
                }
            }
            let pivot_row = match pivot_row {
                None => continue,
                Some(row) => row,
            };
            for k in 0..M {
                let temp = new_matrix.data[current_row][k];
                new_matrix.data[current_row][k] = new_matrix.data[pivot_row][k];
                new_matrix.data[pivot_row][k] = temp;
            }
            let pivot = new_matrix.data[current_row][col];
            for j in 0..M {
                new_matrix.data[current_row][j] = new_matrix.data[current_row][j] / pivot;
            }
            for row in 0..N {
                if row == current_row { continue; }
                let factor = new_matrix.data[row][col];
                for k in 0..M {
                    new_matrix.data[row][k] = new_matrix.data[row][k] - factor * new_matrix.data[current_row][k];
                }
            }
            current_row += 1;
        }
        new_matrix
    }

    pub fn row_echelon_augmented(&self, augmentation : &mut Matrix<K, N , M>) -> Matrix<K, N, M> {
        let mut new_matrix = self.clone();
        let mut current_row = 0;

        for col in 0..M {
            let mut pivot_row = None;
            for row in current_row..N {
                if new_matrix.data[row][col] != K::default() {
                    pivot_row = Some(row);
                    break;
                }
            }
            let pivot_row = match pivot_row {
                None => continue,
                Some(row) => row,
            };
            for k in 0..M {
                let mut temp = new_matrix.data[current_row][k];
                new_matrix.data[current_row][k] = new_matrix.data[pivot_row][k];
                new_matrix.data[pivot_row][k] = temp;

                temp = augmentation.data[current_row][k];
                augmentation.data[current_row][k] = augmentation.data[pivot_row][k];
                augmentation.data[pivot_row][k] = temp;
            }
            let pivot = new_matrix.data[current_row][col];
            for j in 0..M {
                new_matrix.data[current_row][j] = new_matrix.data[current_row][j] / pivot;
                augmentation.data[current_row][j] = augmentation.data[current_row][j] / pivot;

            }
            for row in 0..N {
                if row == current_row { continue; }
                let factor = new_matrix.data[row][col];
                for k in 0..M {
                    new_matrix.data[row][k] = new_matrix.data[row][k] - factor * new_matrix.data[current_row][k];
                    augmentation.data[row][k] = augmentation.data[row][k] - factor * augmentation.data[current_row][k];
                }
            }
            current_row += 1;
        }
        new_matrix
    }
}