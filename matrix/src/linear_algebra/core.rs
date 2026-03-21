use std::ops::{Add, Mul, Sub};

pub fn lerp<V> (u : V, v: V, pct: f32) -> V where V: Sub<Output = V> +  Add<Output = V> + Mul<f32, Output = V> + Copy {
    u + (v - u) * pct
}