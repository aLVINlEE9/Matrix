use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
}

impl<T> Matrix<T>
where
    T: Clone
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + From<f32>
        + Default
        + std::fmt::Display,
{
    pub fn from(data: Vec<Vec<T>>) -> Self {
        Matrix { data }
    }

    pub fn add(&mut self, v: &Matrix<T>) {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            v.data[0].len(),
            "Matrices must have the same dimensions"
        );
        for (row1, row2) in self.data.iter_mut().zip(v.data.iter()) {
            for (a, b) in row1.iter_mut().zip(row2.iter()) {
                *a = a.clone() + b.clone();
            }
        }
    }

    pub fn sub(&mut self, v: &Matrix<T>) {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            v.data[0].len(),
            "Matrices must have the same dimensions"
        );
        for (row1, row2) in self.data.iter_mut().zip(v.data.iter()) {
            for (a, b) in row1.iter_mut().zip(row2.iter()) {
                *a = a.clone() - b.clone();
            }
        }
    }

    pub fn scl(&mut self, a: T) {
        for row in self.data.iter_mut() {
            for x in row.iter_mut() {
                *x = x.clone() * a.clone();
            }
        }
    }
}

impl<T> Add for Matrix<T>
where
    T: Clone + Add<Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Matrices must have the same dimensions"
        );

        Matrix {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(row1, row2)| {
                    row1.iter()
                        .zip(row2.iter())
                        .map(|(a, b)| a.clone() + b.clone())
                        .collect()
                })
                .collect(),
        }
    }
}

impl<T> Sub for Matrix<T>
where
    T: Clone + Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        assert_eq!(
            self.data.len(),
            other.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            other.data[0].len(),
            "Matrices must have the same dimensions"
        );

        Matrix {
            data: self
                .data
                .iter()
                .zip(other.data.iter())
                .map(|(row1, row2)| {
                    row1.iter()
                        .zip(row2.iter())
                        .map(|(a, b)| a.clone() - b.clone())
                        .collect()
                })
                .collect(),
        }
    }
}

impl<T> Mul<T> for Matrix<T>
where
    T: Clone + Mul<Output = T>,
{
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix {
            data: self
                .data
                .iter()
                .map(|row| row.iter().map(|a| a.clone() * rhs.clone()).collect())
                .collect(),
        }
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.data {
            write!(f, "[")?;
            for (i, x) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?
                }
                write!(f, "{:.1}", x)?;
            }
            writeln!(f, "]")?;
        }
        Ok(())
    }
}
