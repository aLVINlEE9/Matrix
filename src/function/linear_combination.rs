use crate::types::vector::Vector;
use std::ops::{Add, Mul, Sub};

pub fn linear_combination<T>(u: &[Vector<T>], coefs: &[T]) -> Vector<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + From<f32>
        + Default
        + std::fmt::Display,
{
    assert_eq!(
        u.len(),
        coefs.len(),
        "Number of vectors and coefficients must match"
    );

    let mut result = Vector::from(vec![T::default(); u[0].data.len()]);

    for (vec, coef) in u.iter().zip(coefs.iter()) {
        for (r, v) in result.data.iter_mut().zip(vec.data.iter()) {
            *r = r.clone() + v.clone() * coef.clone();
        }
    }

    result
}
