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