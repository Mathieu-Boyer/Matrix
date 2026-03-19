mod vector;

use vector::core::Vector;

fn main() {
    let x = Vector::new([1, 2, 3]);

    x.describe();
}
