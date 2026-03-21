use crate::linear_algebra::core::lerp;
use crate::matrix::core::Matrix;
use crate::vector::core::Vector;

pub fn ex00(){
    println!("--------- Ex00 ---------");
    let my_vector = Vector::new([1, 2, 3, 4 , 5 , 6]);
    println!("----- Original vector");
    my_vector.display();
    println!("----- Original vector + Original vector");
    my_vector.add(&my_vector).display();
    println!("----- Original vector - Original vector");
    my_vector.sub(&my_vector).display();
    println!("----- Original vector scaled by 2");
    my_vector.scale(2).display();


    let my_matrix = Matrix::new([[1, 2, 3], [4, 5, 6]]);
    println!("----- Original Matrix");
    my_matrix.display();
    println!("----- Original Matrix + Original Matrix");
    my_matrix.add(&my_matrix).display();
    println!("----- Original Matrix - Original Matrix");
    my_matrix.sub(&my_matrix).display();
    println!("----- Original Matrix scaled by 2");
    my_matrix.scale(2).display();


}

pub fn ex01(){
    println!("--------- Ex01 ---------");

    let e1 = Vector::new([1., 0., 0.]);
    let e2 = Vector::new([0., 1., 0.]);
    let e3 = Vector::new([0., 0., 1.]);
    println!("----- Basis vectors scaled by 10, -2, 0.5");
    Vector::linear_combination(&[e1, e2, e3], &[10., -2., 0.5]).display();

    let v1 = Vector::new([1., 2., 3.]);
    println!("----- Single vector scaled by 2");
    Vector::linear_combination(&[v1], &[2.]).display();

    let v1 = Vector::new([1., 2., 3.]);
    let v2 = Vector::new([0., 10., -100.]);
    println!("----- Two vectors scaled by 10, -2");
    Vector::linear_combination(&[v1, v2], &[10., -2.]).display();

    let v1 = Vector::new([1., 2., 3.]);
    let v2 = Vector::new([4., 5., 6.]);
    println!("----- Zero coefficients");
    Vector::linear_combination(&[v1, v2], &[0., 0.]).display();

    let v1 = Vector::new([1., 2., 3.]);
    let v2 = Vector::new([4., 5., 6.]);
    println!("----- Negative coefficients");
    Vector::linear_combination(&[v1, v2], &[-1., 1.]).display();

}

pub fn ex02(){
    println!("--------- Ex02 ---------");

    println!("----- Scalars");
    println!("{}", lerp(0., 1., 0.));
    println!("{}", lerp(0., 1., 1.));
    println!("{}", lerp(0., 1., 0.5));
    println!("{}", lerp(21., 42., 0.3));

    println!("----- Vectors");
    lerp(Vector::new([2., 1.]), Vector::new([4., 2.]), 0.3).display();
    lerp(Vector::new([0., 0.]), Vector::new([1., 1.]), 0.5).display();

    println!("----- Matrices");
    lerp(Matrix::new([[2., 1.], [3., 4.]]), Matrix::new([[20., 10.], [30., 40.]]), 0.5).display();

}

