mod vector;
pub mod matrix;
pub mod tests;
pub mod linear_algebra;


use linear_algebra::core::lerp;
use crate::vector::core::Vector;

fn main() {
    //tests::exercises::ex00();
    //tests::exercises::ex01();

    let vec1 = Vector::new([0.0, 0.0]);
    let vec2 = Vector::new([1.0, 1.0]);
    lerp(vec1, vec2, 0.5).display();

}
