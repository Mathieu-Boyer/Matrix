use crate::vector::core::Vector;
use std::fmt::Display;
impl<K : Display ,const N: usize> Vector<K,N> {
    pub fn describe(&self){
        print!("Size : {} , Content : [ ", self.size());
        for i in 0..N{
            print!("{}", self.data[i]);
            if i < N - 1{print!(", ")}
        }
        print!(" ]\n");
    }
} 

impl<K, const N: usize> Vector<K,N> {
    pub fn size(&self) -> usize {
        self.data.len()
    }
}