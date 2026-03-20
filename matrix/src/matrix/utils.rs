use std::fmt::Display;
use crate::matrix::core::Matrix;

impl<K : Display, const N : usize, const M : usize> Matrix<K,N, M> {
    pub fn display(&self){
        print!("[\n");
            for i in 0..M {
                print!("[ ");

                for j in 0..N {
                    print!("{}", self.data[i][j]);
                    if j < N - 1 {print!(", ");}

                }

                print!(" ]\n");
            }
        print!("]\n");
    }
}