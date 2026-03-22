use crate::linear_algebra::traits::Field;
use num_traits::One;

#[derive(Copy, Clone)]
pub struct Matrix<K : Field, const N: usize, const M: usize>{
    pub data : [[K; M]; N],
}

impl<K : Field , const N : usize , const M : usize> Matrix<K,N,M> {
    pub fn new(data: [[K; M]; N]) -> Self {
        Matrix { data }
    }

    pub fn identity<const X : usize , const Y: usize>()-> Matrix<K,N,M> {
        let mut mat = Matrix::new([[K::default(); M]; N]);

        for i in 0..N{
            if i < M {mat.data[i][i] = K::one()}
        }

        mat
    }
}