use std::ops::{Add , Sub, Mul};
use crate::vector::core::Vector;

impl<K: Default + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K>, const  N : usize> Vector<K, N>{
    pub fn execute_vector_operation (&self, other : &Vector<K, N>, f : fn(K, K) -> K) -> Vector<K, N> where K : Copy{
        let mut new_vector :  Vector<K, N> = Vector::new([K::default(); N]);

        for i in 0..N{
            new_vector.data[i] = f(self.data[i], other.data[i]);
        }
        new_vector
    }

    pub fn execute_scalar_operation (&self, other : K, f : fn(K, K) -> K) -> Vector<K, N> where K : Copy{
        let mut new_vector :  Vector<K, N> = Vector::new([K::default(); N]);

        for i in 0..N{
            new_vector.data[i] = f(self.data[i], other);
        }
        new_vector
    }

    pub fn add(&self, other : &Vector<K, N>) -> Vector<K, N> where K : Copy {
        self.execute_vector_operation(other, |a, b|{a + b})
    }

    pub fn sub(&self, other : &Vector<K, N>) -> Vector<K, N> where K : Copy {
        self.execute_vector_operation(other, |a, b|{a - b})
    }

    pub fn scale(&self , other : K) -> Vector<K, N> where K : Copy  {
        self.execute_scalar_operation(other, |a, b| { a * b})
    }
}

impl <K , const N : usize> Add for Vector<K, N> where K : Copy + Default + Add<Output = K> + Sub<Output = K> + Mul<Output = K> {
    type Output = Vector<K, N>;
    fn add(self, rhs: Self) -> Self::Output {
        self.execute_vector_operation(&rhs, |a, b| a + b)
    }
}

impl <K , const N : usize> Sub for Vector<K, N> where K : Copy + Default + Add<Output = K> + Sub<Output = K> + Mul<Output = K> {
    type Output = Vector<K, N>;
    fn sub(self, rhs: Self) -> Self::Output {
        self.execute_vector_operation(&rhs, |a, b| a - b)
    }

}

impl<K, const N: usize> Mul<K> for Vector<K, N> where K : Copy + Default + Add<Output = K> + Sub<Output = K> + Mul<Output = K>
{
    type Output = Vector<K, N>;
    fn mul(self, t: K) -> Vector<K, N> {
        // your existing logic

        self.execute_scalar_operation(t, |a, b| a * b)
    }
}