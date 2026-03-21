use crate::linear_algebra::traits::Field;

#[derive(Copy, Clone)]
pub struct Vector<K : Field, const N: usize> {
    pub data: [K; N],
}



impl<K : Field, const N : usize> Vector<K,N> {
    pub fn new(data : [K; N])-> Self {
        Vector { data }
    }
}