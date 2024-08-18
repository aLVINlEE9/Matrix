use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> Vector<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + From<f32>
        + Default
        + std::fmt::Display,
{
    pub fn from(data: Vec<T>) -> Self {
        Vector { data }
    }

    pub fn add(&mut self, v: &Vector<T>) {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same length"
        );
        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a = a.clone() + b.clone();
        }
    }

    pub fn sub(&mut self, v: &Vector<T>) {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same length"
        );
        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a = a.clone() - b.clone();
        }
    }

    pub fn scl(&mut self, a: T) {
        for x in self.data.iter_mut() {
            *x = x.clone() * a.clone();
        }
    }
}

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for x in &self.data {
            writeln!(f, "[{:.1}]", x)?;
        }
        Ok(())
    }
}
