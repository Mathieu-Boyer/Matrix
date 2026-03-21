use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;

impl <K : Field , const N : usize> Vector<K, N>{
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

    pub fn norm (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum + self.data[i].into().powf(2.);
        }
        sum.powf(0.5)
    }

    pub fn norm_inf (&self) -> f32 {
        let mut sum: f32 = 0.;
        for i in 0..N {
            sum = sum.max(Self::abs(self.data[i]).into());
        }
        sum
    }


}