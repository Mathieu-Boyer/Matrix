use std::ops::{Add, Mul};
use crate::vector::core::Vector;

impl<K , const N : usize> Vector<K, N> where K : Default + Copy + Mul<Output = K> + Add<Output = K>{
    pub fn dot(&self,other:&Vector<K, N>) -> K{

        let mut sum = K::default();

        for i in 0..N{
            sum = sum + (self.data[i] * other.data[i]);
        }
        sum
    }
}