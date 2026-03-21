use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub trait Field :
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Neg<Output = Self> +
    PartialOrd +
    Default +
    Display +
    Copy +
    Into<f32>{}

impl Field for f32 {}
