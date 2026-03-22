use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};
use num_traits::One;

pub(crate) use crate::linear_algebra::types::Complex;

pub trait Field :
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Neg<Output = Self> +
    Default +
    Display +
    PartialEq +
    Copy +
    Into<f32> +
    One
    {}

pub trait Real : Field + PartialOrd {}

impl Real for f32 {}

impl Field for f32 {}


impl Add for Complex {
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex {
            real : self.real + rhs.real,
            i : self.i + rhs.i,
        }
    }
}

impl Sub for Complex {
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex {
            real : self.real - rhs.real,
            i : self.i - rhs.i,
        }
    }
}

impl Mul for Complex {
    type Output = Complex;
    fn mul(self, other: Complex) -> Complex {
        Complex {
            real: self.real * other.real - self.i * other.i,
            i: self.real * other.i + self.i * other.real,
        }
    }
}

impl Div for Complex {
    type Output = Complex;
    fn div(self, other: Complex) -> Complex {
        let denominator = other.real * other.real + other.i * other.i;
        Complex {
            real: (self.real * other.real + self.i * other.i) / denominator,
            i: (self.i * other.real - self.real * other.i) / denominator,
        }
    }
}
impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Complex {
        Complex {
            real: -self.real,
            i: -self.i
        }
    }
}

impl Default for Complex {
    fn default() -> Complex {
        Complex {
            real: 0.,
            i: 0.
        }
    }
}

impl One for Complex {
    fn one() -> Complex {
        Complex {
            real: 1.0,
            i: 0.0,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} + {}*i)", self.real, self.i)
    }
}

impl PartialEq for Complex {
    fn eq(&self, other: &Complex) -> bool {
        self.real == other.real && self.i == other.i
    }
}

impl Into<f32> for Complex {
    fn into(self) -> f32 {
        (self.real * self.real + self.i * self.i).powf(0.5)
    }
}
impl Mul<f32> for Complex {
    type Output = Complex;
    fn mul(self, t: f32) -> Complex {
        Complex {
            real: self.real * t,
            i: self.i * t,
        }
    }
}
impl Field for Complex {}
