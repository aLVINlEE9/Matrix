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

        for (row1, row2) in self.data.iter_mut().zip(v.data.iter()) {
            for (a, b) in row1.iter_mut().zip(row2.iter()) {
                *a = a.clone() + b.clone();
            }
        }
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

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            for (a, b) in a.iter_mut().zip(b.iter()) {
                *a = a.clone() - b.clone();
            }
        }
        self
    }

    pub fn scl(&mut self, a: T) -> &Self {
        for row in self.data.iter_mut() {
            for x in row.iter_mut() {
                *x = x.clone() * a.clone();
            }
        }
        self
    }
}

// impl<T> Add for Matrix<T>
// where
//     T: Add<Output = T> + Clone,
// {
//     type Output = Self;
//     fn add(mut self, m: Self) -> Self::Output {
//         assert_eq!(
//             self.data.len(),
//             m.data.len(),
//             "Matrices must have the same dimensions"
//         );
//         assert_eq!(
//             self.data[0].len(),
//             m.data[0].len(),
//             "Matrices must have the same dimensions"
//         );

//         for (row1, row2) in self.data.iter_mut().zip(m.data.iter()) {
//             for (a, b) in row1.iter_mut().zip(row2.iter()) {
//                 *a = a.clone() + b.clone();
//             }
//         }
//         self
//     }
// }

// impl<T> Sub for Matrix<T>
// where
//     T: Sub<Output = T> + Clone,
// {
//     type Output = Self;
//     fn sub(mut self, m: Self) -> Self::Output {
//         assert_eq!(
//             self.data.len(),
//             m.data.len(),
//             "Matrices must have the same dimensions"
//         );
//         assert_eq!(
//             self.data[0].len(),
//             m.data[0].len(),
//             "Matrices must have the same dimensions"
//         );

//         for (row1, row2) in self.data.iter_mut().zip(m.data.iter()) {
//             for (a, b) in row1.iter_mut().zip(row2.iter()) {
//                 *a = a.clone() - b.clone();
//             }
//         }
//         self
//     }
// }

// impl<T> Matrix<T>
// where
//     T: Mul<Output = T> + Clone,
// {
//     pub fn scl(&mut self, a: T) {
//         for row in self.data.iter_mut() {
//             for x in row.iter_mut() {
//                 *x = x.clone() * a.clone();
//             }
//         }
//     }
// }
