use crate::matrix::core::Matrix;

impl<K: std::ops::Add<Output = K> + std::ops::Sub<Output = K> + Default + std::ops::Mul<Output = K>, const N : usize , const M : usize> Matrix<K, N, M>{
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