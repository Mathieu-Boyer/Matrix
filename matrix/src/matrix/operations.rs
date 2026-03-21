use std::ops::{Add, Sub, Mul};
use crate::linear_algebra::traits::Field;
use crate::matrix::core::Matrix;

impl<K : Field, const N : usize , const M : usize> Matrix<K, N, M> where K : Add<Output = K> + Sub<Output = K> + Default + Mul<Output = K>{
    fn execute_matrix_operation(&self , other : &Matrix<K, N, M>, f : fn(K, K) -> K) -> Matrix<K, N, M> where K : Copy {
        let mut new_mat = Matrix::<K, N, M>::new([[K::default(); M] ; N]);
        for i in 0..N{
            for j in 0..M{
                new_mat.data[i][j] = f(self.data[i][j], other.data[i][j]);
            }
        }
        new_mat
    }

    fn execute_scalar_operation(&self , other : K, f : fn(K, K) -> K) -> Matrix<K, N, M> where K : Copy {
        let mut new_mat = Matrix::<K, N, M>::new([[K::default(); M] ; N]);
        for i in 0..N{
            for j in 0..M{
                new_mat.data[i][j] = f(self.data[i][j], other)
            }
        }

        new_mat
    }

    pub fn add(&self, other : &Matrix<K, N, M>) -> Matrix<K, N , M>  where K : Copy {
        self.execute_matrix_operation(&other, |a, b| a + b)
    }

    pub fn sub(&self, other : &Matrix<K, N, M>)-> Matrix<K, N , M>   where K : Copy {
        self.execute_matrix_operation(&other, |a, b| a - b)
    }

    pub fn scale(&self, factor : K) -> Matrix<K, N , M>   where K : Copy {
        self.execute_scalar_operation(factor, |a, b| a * b)
    }
}


impl<K : Field, const N : usize> Add for Matrix<K, N, N>where K : Add<Output = K> + Sub<Output = K> + Default + Mul<Output = K> + Copy {
    type Output = Matrix<K, N, N>;

    fn add(self, rhs: Self) -> Self::Output {
        self.execute_matrix_operation(&rhs, |a, b| a + b)
    }
}

impl<K : Field, const N : usize> Sub for Matrix<K, N, N>where K : Add<Output = K> + Sub<Output = K> + Default + Mul<Output = K> + Copy {
    type Output = Matrix<K, N, N>;

    fn sub(self, rhs: Self) -> Self::Output {
        self.execute_matrix_operation(&rhs, |a, b| a - b)
    }
}

impl<K : Field, const N : usize> Mul<K> for Matrix<K, N, N>where K : Add<Output = K> + Sub<Output = K> + Default + Mul<Output = K> + Copy {
    type Output = Matrix<K, N, N>;

    fn mul(self, rhs: K) -> Self::Output {
        self.execute_scalar_operation(rhs, |a, b| a * b)
    }
}