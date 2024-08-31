use std::fmt;
use std::ops::{Add, Mul, Sub};

pub type Array2D<T> = Vec<Vec<T>>;

#[derive(Clone, Debug)]
pub struct Matrix<T> {
    pub data: Array2D<T>,
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display,
{
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

impl<T> From<Array2D<T>> for Matrix<T> {
    fn from(value: Array2D<T>) -> Self {
        Matrix { data: value }
    }
}

impl<T> Matrix<T>
where
    T: Copy + Default,
{
    pub fn add(&mut self, v: &Matrix<T>) -> &Self
    where
        T: Add<Output = T>,
    {
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

        self.data
            .iter_mut()
            .zip(v.data.iter())
            .for_each(|(row1, row2)| {
                row1.iter_mut().zip(row2.iter()).for_each(|(a, b)| {
                    *a = *a + *b;
                });
            });

        self
    }

    pub fn sub(&mut self, v: &Matrix<T>) -> &Self
    where
        T: Sub<Output = T>,
    {
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

        self.data
            .iter_mut()
            .zip(v.data.iter())
            .for_each(|(row1, row2)| {
                row1.iter_mut().zip(row2.iter()).for_each(|(a, b)| {
                    *a = *a - *b;
                });
            });

        self
    }

    pub fn scl(&mut self, a: T) -> &Self
    where
        T: Mul<Output = T>,
    {
        self.data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|x| {
                *x = *x * a;
            });
        });

        self
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Copy,
{
    type Output = Self;
    fn add(mut self, m: Self) -> Self::Output {
        assert_eq!(
            self.data.len(),
            m.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            m.data[0].len(),
            "Matrices must have the same dimensions"
        );

        self.data
            .iter_mut()
            .zip(m.data.iter())
            .for_each(|(row1, row2)| {
                row1.iter_mut().zip(row2.iter()).for_each(|(a, b)| {
                    *a = *a + *b;
                });
            });
        self
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Copy,
{
    type Output = Self;
    fn sub(mut self, m: Self) -> Self::Output {
        assert_eq!(
            self.data.len(),
            m.data.len(),
            "Matrices must have the same dimensions"
        );
        assert_eq!(
            self.data[0].len(),
            m.data[0].len(),
            "Matrices must have the same dimensions"
        );

        self.data
            .iter_mut()
            .zip(m.data.iter())
            .for_each(|(row1, row2)| {
                row1.iter_mut().zip(row2.iter()).for_each(|(a, b)| {
                    *a = *a - *b;
                });
            });
        self
    }
}

impl<T> Mul<f64> for Matrix<T>
where
    T: Mul<f64, Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.data.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|val| {
                *val = *val * rhs;
            });
        });
        self
    }
}
