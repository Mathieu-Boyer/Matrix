use crate::linear_algebra::core::{angle_cos, cross_product, lerp, projection};
use crate::matrix::core::Matrix;
use crate::vector::core::Vector;
use crate::linear_algebra::types::Complex;

pub fn ex00(){
    println!("--------- Ex00 ---------");
    let my_vector = Vector::new([1., 2., 3., 4. , 5. , 6.]);
    println!("----- Original vector");
    my_vector.display();
    println!("----- Original vector + Original vector");
    my_vector.add(&my_vector).display();
    println!("----- Original vector - Original vector");
    my_vector.sub(&my_vector).display();
    println!("----- Original vector scaled by 2");
    my_vector.scale(2.).display();


    let my_matrix = Matrix::new([[1., 2., 3.], [4., 5., 6.]]);
    println!("----- Original Matrix");
    my_matrix.display();
    println!("----- Original Matrix + Original Matrix");
    my_matrix.add(&my_matrix).display();
    println!("----- Original Matrix - Original Matrix");
    my_matrix.sub(&my_matrix).display();
    println!("----- Original Matrix scaled by 2");
    my_matrix.scale(2.).display();


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


pub fn ex03(){
    println!("--------- Ex03 ---------");

    println!("----- Zero vectors");
    println!("{}", Vector::new([0., 0.]).dot(&Vector::new([1., 1.])));

    println!("----- Same vectors");
    println!("{}", Vector::new([1., 1.]).dot(&Vector::new([1., 1.])));

    println!("----- Opposite vectors");
    println!("{}", Vector::new([-1., 6.]).dot(&Vector::new([3., 2.])));

    println!("----- 3D vectors");
    println!("{}", Vector::new([1., 2., 3.]).dot(&Vector::new([4., 5., 6.])));

    println!("----- Negative values");
    println!("{}", Vector::new([-1., -2., -3.]).dot(&Vector::new([1., 2., 3.])));
}

pub fn ex04(){
    println!("--------- Ex04 ---------");

    println!("----- Zero vector");
    let u = Vector::new([0., 0., 0.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());

    println!("----- Positive values");
    let u = Vector::new([1., 2., 3.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());

    println!("----- Negative values");
    let u = Vector::new([-1., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());

    println!("----- Single value");
    let u = Vector::new([5.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());

    println!("----- Mixed values");
    let u = Vector::new([-3., 1., -5., -2.]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());
}

pub fn ex05(){
    println!("--------- Ex05 ---------");

    println!("----- Same vectors");
    println!("{}", angle_cos(&Vector::new([1., 0.]), &Vector::new([1., 0.])));

    println!("----- Perpendicular vectors");
    println!("{}", angle_cos(&Vector::new([1., 0.]), &Vector::new([0., 1.])));

    println!("----- Opposite vectors");
    println!("{}", angle_cos(&Vector::new([-1., 1.]), &Vector::new([1., -1.])));

    println!("----- Colinear vectors");
    println!("{}", angle_cos(&Vector::new([2., 1.]), &Vector::new([4., 2.])));

    println!("----- 3D vectors");
    println!("{}", angle_cos(&Vector::new([1., 2., 3.]), &Vector::new([4., 5., 6.])));

}

pub fn ex06(){
    println!("--------- Ex06 ---------");

    println!("----- Standard basis");
    cross_product(&Vector::new([0., 0., 1.]), &Vector::new([1., 0., 0.])).display();

    println!("----- Subject example 2");
    cross_product(&Vector::new([1., 2., 3.]), &Vector::new([4., 5., 6.])).display();

    println!("----- Subject example 3");
    cross_product(&Vector::new([4., 2., -3.]), &Vector::new([-2., -5., 16.])).display();

    println!("----- Parallel vectors");
    cross_product(&Vector::new([1., 0., 0.]), &Vector::new([2., 0., 0.])).display();

    println!("----- Opposite vectors");
    cross_product(&Vector::new([1., 2., 3.]), &Vector::new([-1., -2., -3.])).display();
}

pub fn ex07(){
    println!("--------- Ex07 ---------");

    println!("----- Identity * vector");
    Matrix::new([[1., 0.], [0., 1.]]).mul_vec(&Vector::new([4., 2.])).display();

    println!("----- Scale matrix * vector");
    Matrix::new([[2., 0.], [0., 2.]]).mul_vec(&Vector::new([4., 2.])).display();

    println!("----- Mixed matrix * vector");
    Matrix::new([[2., -2.], [-2., 2.]]).mul_vec(&Vector::new([4., 2.])).display();

    println!("----- Identity * Identity");
    Matrix::new([[1., 0.], [0., 1.]]).mul_mat(&Matrix::new([[1., 0.], [0., 1.]])).display();

    println!("----- Identity * matrix");
    Matrix::new([[1., 0.], [0., 1.]]).mul_mat(&Matrix::new([[2., 1.], [4., 2.]])).display();

    println!("----- Matrix * matrix");
    Matrix::new([[3., -5.], [6., 8.]]).mul_mat(&Matrix::new([[2., 1.], [4., 2.]])).display();
}


pub fn ex08(){
    println!("--------- Ex08 ---------");

    println!("----- Identity matrix");
    println!("{}", Matrix::new([[1., 0.], [0., 1.]]).trace());

    println!("----- 3x3 matrix");
    println!("{}", Matrix::new([[2., -5., 0.], [4., 3., 7.], [-2., 3., 4.]]).trace());

    println!("----- Negative trace");
    println!("{}", Matrix::new([[-2., -8., 4.], [1., -23., 4.], [0., 6., 4.]]).trace());

    println!("----- Zero matrix");
    println!("{}", Matrix::new([[0., 0.], [0., 0.]]).trace());

    println!("----- Single element");
    println!("{}", Matrix::new([[5.]]).trace());
}

pub fn ex09(){
    println!("--------- Ex09 ---------");

    println!("----- Identity matrix");
    Matrix::new([[1., 0.], [0., 1.]]).transpose().display();

    println!("----- Square matrix");
    Matrix::new([[1., 2.], [3., 4.]]).transpose().display();

    println!("----- Rectangular matrix");
    Matrix::new([[1., 2., 3.], [4., 5., 6.]]).transpose().display();

    println!("----- Single row");
    Matrix::new([[1., 2., 3.]]).transpose().display();

    println!("----- Single column");
    Matrix::new([[1.], [2.], [3.]]).transpose().display();
}

pub fn ex10(){
    println!("--------- Ex10 ---------");

    println!("----- Identity matrix");
    Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).row_echelon().display();

    println!("----- Simple 2x2");
    Matrix::new([[1., 2.], [3., 4.]]).row_echelon().display();

    println!("----- Singular 2x2");
    Matrix::new([[1., 2.], [2., 4.]]).row_echelon().display();

    println!("----- Rectangular 3x5");
    Matrix::new([
        [8., 5., -2., 4., 28.],
        [4., 2.5, 20., 4., -4.],
        [8., 5., 1., 4., 17.]
    ]).row_echelon().display();

    println!("----- Singular 3x3");
    Matrix::new([[1., 2., 3.], [2., 4., 6.], [3., 6., 9.]]).row_echelon().display();

    println!("----- Zero matrix");
    Matrix::new([[0., 0.], [0., 0.]]).row_echelon().display();

    println!("----- Single row");
    Matrix::new([[1., 2., 3.]]).row_echelon().display();
}

pub fn ex11(){
    println!("--------- Ex11 ---------");

    println!("----- Identity 2x2");
    println!("{}", Matrix::new([[1., 0.], [0., 1.]]).determinant());

    println!("----- Singular 2x2");
    println!("{}", Matrix::new([[1., -1.], [-1., 1.]]).determinant());

    println!("----- Scale 3x3");
    println!("{}", Matrix::new([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]).determinant());

    println!("----- 3x3");
    println!("{}", Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]).determinant());

    println!("----- 4x4");
    println!("{}", Matrix::new([[8., 5., -2., 4.], [4., 2.5, 20., 4.], [8., 5., 1., 4.], [28., -4., 17., 1.]]).determinant());

    println!("----- Single element");
    println!("{}", Matrix::new([[5.]]).determinant());

    println!("----- Zero matrix");
    println!("{}", Matrix::new([[0., 0.], [0., 0.]]).determinant());
}

pub fn ex12(){
    println!("--------- Ex12 ---------");

    println!("----- Identity matrix");
    match Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }

    println!("----- Scale matrix");
    match Matrix::new([[2., 0., 0.], [0., 2., 0.], [0., 0., 2.]]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }

    println!("----- 3x3");
    match Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.]]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }

    println!("----- Singular matrix");
    match Matrix::new([[1., -1.], [-1., 1.]]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }

    println!("----- Single element");
    match Matrix::new([[5.]]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }
}

pub fn ex13 (){
    println!("--------- Ex13 ---------");

    println!("----- Identity matrix");
    println!("{}", Matrix::new([[1., 0., 0.], [0., 1., 0.], [0., 0., 1.]]).rank());

    println!("----- Full rank rectangular");
    println!("{}", Matrix::new([[1., 2., 0., 0.], [2., 4., 0., 0.], [-1., 2., 1., 1.]]).rank());

    println!("----- Full rank 4x3");
    println!("{}", Matrix::new([[8., 5., -2.], [4., 7., 20.], [7., 6., 1.], [21., 18., 7.]]).rank());

    println!("----- Singular matrix");
    println!("{}", Matrix::new([[1., -1.], [-1., 1.]]).rank());

    println!("----- Zero matrix");
    println!("{}", Matrix::new([[0., 0.], [0., 0.]]).rank());

    println!("----- Single element");
    println!("{}", Matrix::new([[5.]]).rank());
}


pub fn ex14(){
    println!("--------- Ex14 ---------");
    let to_format = projection(1., 1., 0.1, 100.0);

    for i in 0..4{
        for j in 0..4{
            print!("{}", to_format.data[i][j]);
            if j == 4 - 1{
                print!("\n");
            }else{
                print!(", ");
            }
        }
    }
}

pub fn ex15(){
    println!("--------- Ex15 ---------");

    println!("----- Ex00 - Add Sub Scale");
    let u = Vector::new([Complex { real: 1., i: 2. }, Complex { real: 3., i: 4. }]);
    let v = Vector::new([Complex { real: 5., i: 6. }, Complex { real: 7., i: 8. }]);
    u.add(&v).display();
    u.sub(&v).display();
    u.scale(Complex { real: 2., i: 0. }).display();

    println!("----- Ex01 - Linear combination");
    let e1 = Vector::new([Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }]);
    let e2 = Vector::new([Complex { real: 0., i: 0. }, Complex { real: 1., i: 0. }]);
    Vector::linear_combination(&[e1, e2], &[Complex { real: 2., i: 1. }, Complex { real: 3., i: -1. }]).display();

    println!("----- Ex02 - Lerp");
    println!("{}", lerp(Complex { real: 0., i: 0. }, Complex { real: 1., i: 1. }, 0.5));

    println!("----- Ex03 - Dot product");
    let u = Vector::new([Complex { real: 1., i: 0. }, Complex { real: 0., i: 1. }]);
    let v = Vector::new([Complex { real: 1., i: 0. }, Complex { real: 0., i: 1. }]);
    println!("{}", u.dot(&v));

    println!("----- Ex04 - Norms");
    let u = Vector::new([Complex { real: 3., i: 4. }, Complex { real: 0., i: 0. }]);
    println!("{}, {}, {}", u.norm_1(), u.norm(), u.norm_inf());

    println!("----- Ex05 - Cosine");
    let u = Vector::new([Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }]);
    let v = Vector::new([Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }]);
    println!("{}", angle_cos(&u, &v));

    println!("----- Ex07 - Matrix multiplication");
    let m = Matrix::new([[Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }], [Complex { real: 0., i: 0. }, Complex { real: 1., i: 0. }]]);
    let v = Vector::new([Complex { real: 3., i: 1. }, Complex { real: 2., i: -1. }]);
    m.mul_vec(&v).display();

    println!("----- Ex08 - Trace");
    let m = Matrix::new([
        [Complex { real: 1., i: 2. }, Complex { real: 0., i: 0. }],
        [Complex { real: 0., i: 0. }, Complex { real: 3., i: 4. }]
    ]);
    println!("{}", m.trace());

    println!("----- Ex09 - Transpose");
    let m = Matrix::new([
        [Complex { real: 1., i: 2. }, Complex { real: 3., i: 4. }],
        [Complex { real: 5., i: 6. }, Complex { real: 7., i: 8. }]
    ]);
    m.transpose().display();

    println!("----- Ex10 - Row echelon");
    let m = Matrix::new([
        [Complex { real: 1., i: 0. }, Complex { real: 2., i: 0. }],
        [Complex { real: 3., i: 0. }, Complex { real: 4., i: 0. }]
    ]);
    m.row_echelon().display();

    println!("----- Ex11 - Determinant");
    let m = Matrix::new([
        [Complex { real: 1., i: 0. }, Complex { real: 2., i: 0. }],
        [Complex { real: 3., i: 0. }, Complex { real: 4., i: 0. }]
    ]);
    println!("{}", m.determinant());

    println!("----- Ex12 - Inverse");
    match Matrix::new([
        [Complex { real: 2., i: 0. }, Complex { real: 0., i: 0. }],
        [Complex { real: 0., i: 0. }, Complex { real: 2., i: 0. }]
    ]).inverse() {
        Ok(m) => m.display(),
        Err(e) => println!("{}", e),
    }

    println!("----- Ex13 - Rank");
    let m = Matrix::new([
        [Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }, Complex { real: 0., i: 0. }],
        [Complex { real: 0., i: 0. }, Complex { real: 1., i: 0. }, Complex { real: 0., i: 0. }],
        [Complex { real: 0., i: 0. }, Complex { real: 0., i: 0. }, Complex { real: 1., i: 0. }]
    ]);
    println!("{}", m.rank());
}