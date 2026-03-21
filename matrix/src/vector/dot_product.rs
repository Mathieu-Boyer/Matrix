use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;

impl<K : Field, const N : usize> Vector<K, N>{
    pub fn dot(&self,other:&Vector<K, N>) -> K{

        let mut sum = K::default();

        for i in 0..N{
            sum = sum + (self.data[i] * other.data[i]);
        }
        sum
    }
}