
#[derive(Copy, Clone)]
pub struct Matrix<K, const N: usize, const M: usize>{
    pub data : [[K; M]; N],
}

impl<K , const N : usize , const M : usize> Matrix<K,N,M> {
    pub fn new(data: [[K; M]; N]) -> Self {
        Matrix { data }
    }
}