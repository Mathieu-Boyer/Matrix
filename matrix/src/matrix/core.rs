use crate::linear_algebra::traits::Field;

#[derive(Copy, Clone)]
pub struct Matrix<K : Field, const N: usize, const M: usize>{
    pub data : [[K; M]; N],
}

impl<K : Field , const N : usize , const M : usize> Matrix<K,N,M> {
    pub fn new(data: [[K; M]; N]) -> Self {
        Matrix { data }
    }
}