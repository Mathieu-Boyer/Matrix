use crate::vector::core::Vector;

impl<K: Default + std::ops::Add<Output = K> + std::ops::Sub<Output = K> + std::ops::Mul<Output = K>, const  N : usize> Vector<K, N>{
    fn execute_vector_operation (&self, other : &Vector<K, N>, f : fn(K, K) -> K) -> Vector<K, N> where K : Copy{
        let mut new_vector :  Vector<K, N> = Vector::new([K::default(); N]);

        for i in 0..N{
            new_vector.data[i] = f(self.data[i], other.data[i]);
        }
        new_vector
    }

    fn execute_scalar_operation (&self, other : K,  f : fn(K, K) -> K) -> Vector<K, N> where K : Copy{
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