use std::ops::{Add, Mul, Sub};
use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;

pub fn lerp<V> (u : V, v: V, pct: f32) -> V where V: Sub<Output = V> +  Add<Output = V> + Mul<f32, Output = V> + Copy {
    u + (v - u) * pct
}

pub fn angle_cos<K : Field, const N: usize>(v1: &Vector<K, N>,v2: &Vector<K, N>) -> f32 {
    v1.dot(&v2).into() / (v1.norm() * v2.norm())
}

pub fn cross_product<K : Field, const N : usize>(v1: &Vector<K, N>, v2: &Vector<K, N>) -> Vector<K, N> {
    v1.cross(&v2)
}