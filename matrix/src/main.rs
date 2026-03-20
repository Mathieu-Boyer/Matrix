mod vector;
pub mod matrix;

use vector::core::Vector;
use matrix::core::Matrix;

fn main() {
    let x = Vector::new([1, 2, 3]);
    let testMatrix = Matrix::new([
        [1, 2, 3],
        [4, 5, 6],
    ]);

    testMatrix.display();
    x.describe();
}
