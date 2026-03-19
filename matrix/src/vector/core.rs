
pub struct Vector<K, const N: usize> {
    pub data: [K; N],
}

impl<K, const N : usize> Vector<K,N> {
    pub fn new(data : [K; N])-> Self {
        Vector { data }
    }
}