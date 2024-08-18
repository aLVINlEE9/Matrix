use std::ops::{Add, Mul, Sub};

pub fn lerp<T>(u: T, v: T, t: f64) -> T
where
    T: Add<Output = T> + Sub<Output = T> + Mul<f64, Output = T> + Clone,
{
    u.clone() * (1.0 - t) + v.clone() * t
}
