mod vector;
pub mod matrix;

use vector::core::Vector;
use matrix::core::Matrix;

fn main() {
    let test_matrix1 = Matrix::new([
        [1, 2, 3],
        [4, 5, 6],
    ]);

    let _test_matrix2 = Matrix::new([
        [1],
    ]);

    let test_vector1 = Vector::new([1, 2, 3, 4, 5, 6]);

    test_vector1.to_matrix::<1,6>().display();

    test_matrix1.to_vec::<6>().display()
}
