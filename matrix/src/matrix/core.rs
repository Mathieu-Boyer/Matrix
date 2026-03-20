
pub struct Matrix<K, const N: usize, const M: usize>{
    pub data : [[K; N]; M],
}



impl<K , const N : usize , const M : usize> Matrix<K,N,M>{
    pub fn new(data : [[K; N]; M])-> Self {
        Matrix { data }
    }
}