use crate::linear_algebra::traits::{Field, Real};
use crate::linear_algebra::types::Complex;
use crate::vector::core::Vector;


impl <K : Field, const N : usize> Vector<K, N> {
    pub fn norm (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum + self.data[i].into().powf(2.);

        }
        sum.powf(0.5)
    }
}
impl <K : Real , const N : usize> Vector<K, N>{
    fn abs(num : K) -> K {
        if num < K::default() {-num} else {num}
    }
    pub fn norm_1 (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum + Self::abs(self.data[i]).into();
        }
        sum
    }



    pub fn norm_inf (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum.max(Self::abs(self.data[i]).into());
        }
        sum
    }


}


impl <const N : usize> Vector<Complex, N>{
    pub fn norm_1 (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            let to_float: f32 = self.data[i].into();
            sum += to_float;
        }
        sum
    }


    pub fn norm_inf (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum.max(self.data[i].into())
        }
        sum
    }
}