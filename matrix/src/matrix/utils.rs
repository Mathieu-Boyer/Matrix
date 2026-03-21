use std::fmt::Display;
use crate::matrix::core::Matrix;
use crate::vector::core::Vector;

impl<K : Display, const N : usize, const M : usize> Matrix<K,N, M> {
    pub fn display(&self){
        println!("Shape : ({}, {})", N, M);
        print!("[\n");
            for i in 0..N {
                print!("[ ");

                for j in 0..M {
                    print!("{}", self.data[i][j]);
                    if j < M - 1 {print!(", ");}

                }

                print!(" ]\n");
            }
        print!("]\n");
        println!("Matrix is square ? {}", self.is_square());

    }

    pub fn shape(&self) -> (usize, usize){
        (N, M)
    }
    pub fn is_square(&self) -> bool {
        N == M
    }


    pub fn to_vec<const SIZE : usize>(&self) -> Vector<K, SIZE> where K : Copy {

        if M * N != SIZE{
            panic!("Error : Incompatible size.")
        }

        let data = std::array::from_fn(|i| {
            let col = i / M;
            let row = i % M;
            self.data[col][row]
        });

        Vector { data }
    }
}