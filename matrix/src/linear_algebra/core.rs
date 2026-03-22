use std::ops::{Add, Mul, Sub};
use crate::linear_algebra::traits::Field;
use crate::vector::core::Vector;
use crate::matrix::core::Matrix;

pub fn lerp<V> (u : V, v: V, pct: f32) -> V where V: Sub<Output = V> +  Add<Output = V> + Mul<f32, Output = V> + Copy {
    u + (v - u) * pct
}

pub fn angle_cos<K : Field, const N: usize>(v1: &Vector<K, N>,v2: &Vector<K, N>) -> f32 {
    v1.dot(&v2).into() / (v1.norm() * v2.norm())
}

pub fn cross_product<K : Field, const N : usize>(v1: &Vector<K, N>, v2: &Vector<K, N>) -> Vector<K, N> {
    v1.cross(&v2)
}

pub fn projection(fov: f32, ratio: f32, near: f32, far: f32) -> Matrix<f32, 4, 4>{

    let f = 1. / (fov / 2.).tan();
    let x_scale = f / ratio;
    let y_scale = f;
    let z_scale = far / (near - far);
    let z_offset = (near * far) / (near - far);

    let proj_mat = Matrix::new([
        [x_scale, 0., 0., 0.],
        [0., y_scale, 0., 0.],
        [0., 0., z_scale, -1.],
        [0., 0., z_offset, 0.]
    ]);

    proj_mat
}