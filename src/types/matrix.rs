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
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn add(&mut self, v: &Matrix<T>) -> &Self {
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
        self.data = self
            .data
            .iter()
            .zip(v.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a.clone() + b.clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();

        self
    }

    pub fn sub(&mut self, v: &Matrix<T>) -> &Self {
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

        self.data = self
            .data
            .iter()
            .zip(v.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a.clone() - b.clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();
        self
    }

    pub fn scl(&mut self, a: T) -> &Self {
        self.data = self
            .data
            .iter()
            .map(|row| {
                row.iter()
                    .map(|x| x.clone() * a.clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();
        self
    }
}

impl<T> Add for Matrix<T>
where
    T: Add<Output = T> + Clone,
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

        self.data = self
            .data
            .iter()
            .zip(m.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a.clone() + b.clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();
        self
    }
}

impl<T> Sub for Matrix<T>
where
    T: Sub<Output = T> + Clone,
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

        self.data = self
            .data
            .iter()
            .zip(m.data.iter())
            .map(|(row1, row2)| {
                row1.iter()
                    .zip(row2.iter())
                    .map(|(a, b)| a.clone() - b.clone())
                    .collect::<Vec<T>>()
            })
            .collect::<Vec<Vec<T>>>();
        self
    }
}

impl<T: Copy + Mul<f64, Output = T>> Mul<f64> for Matrix<T> {
    type Output = Matrix<T>;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.data = self
            .data
            .into_iter()
            .map(|row| row.into_iter().map(|val| val * rhs).collect::<Vec<T>>())
            .collect::<Vec<Vec<T>>>();
        self
    }
}
