use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug)]
pub struct Vector<T> {
    pub data: Vec<T>,
}

impl<T> fmt::Display for Vector<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        for (i, x) in self.data.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:.1}", x)?;
        }
        writeln!(f, "]")?;
        Ok(())
    }
}

impl<T> From<Vec<T>> for Vector<T> {
    fn from(value: Vec<T>) -> Self {
        Vector { data: value }
    }
}

impl<T> Vector<T>
where
    T: Clone + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn add(&mut self, v: &Vector<T>) -> &Self {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same dimensions"
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a = a.clone() + b.clone();
        }
        self
    }

    pub fn sub(&mut self, v: &Vector<T>) -> &Self {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same dimensions"
        );

        for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
            *a = a.clone() - b.clone();
        }
        self
    }

    pub fn scl(&mut self, a: T) -> &Self {
        for x in self.data.iter_mut() {
            *x = x.clone() * a.clone();
        }
        self
    }
}

// impl<T> Add for Vector<T>
// where
//     T: Add<Output = T> + Clone,
// {
//     type Output = Self;
//     fn add(mut self, v: Self) -> Self::Output {
//         assert_eq!(
//             self.data.len(),
//             v.data.len(),
//             "Vectors must have the same dimensions"
//         );

//         for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
//             *a = a.clone() + b.clone();
//         }
//         self
//     }
// }

// impl<T> Sub for Vector<T>
// where
//     T: Sub<Output = T> + Clone,
// {
//     type Output = Self;
//     fn sub(mut self, v: Self) -> Self::Output {
//         assert_eq!(
//             self.data.len(),
//             v.data.len(),
//             "Vectors must have the same dimensions"
//         );

//         for (a, b) in self.data.iter_mut().zip(v.data.iter()) {
//             *a = a.clone() - b.clone();
//         }
//         self
//     }
// }

// impl<T> Mul<T> for Vector<T>
// where
//     T: Mul<Output = T> + Clone,
// {
//     type Output = Self;
//     fn mul(mut self, scalar: T) -> Self::Output {
//         for a in self.data.iter_mut() {
//             *a = a.clone() * scalar.clone();
//         }
//         self
//     }
// }
