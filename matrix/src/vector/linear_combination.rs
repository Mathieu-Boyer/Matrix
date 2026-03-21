use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;

impl <K: Field, const N : usize> Vector<K, N> {
    pub fn linear_combination(u : &[Vector<K, N>], coefficients : &[K]) -> Vector<K, N> where K : Copy {
        if u.len() != coefficients.len() { panic!("There should be as many vectors as coefficients.")}

        let mut new_vector = Vector::new( [K::default(); N]);

        for i in 0..u.len() {
            for j in 0..N{
                new_vector.data[j] = u[i].data[j] * coefficients[i] + new_vector.data[j];
            }
        }
        new_vector
    }
}