use std::fmt;
use std::iter::Sum;
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

impl<T, const N: usize> From<[T; N]> for Vector<T>
where
    T: Clone,
{
    fn from(value: [T; N]) -> Self {
        Vector {
            data: value.to_vec(),
        }
    }
}

impl<T> Vector<T>
where
    T: Copy + Default,
{
    pub fn add(&mut self, v: &Vector<T>) -> &Self
    where
        T: Add<Output = T>,
    {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same dimensions"
        );

        self.data.iter_mut().zip(v.data.iter()).for_each(|(a, b)| {
            *a = *a + *b;
        });
        self
    }

    pub fn sub(&mut self, v: &Vector<T>) -> &Self
    where
        T: Sub<Output = T>,
    {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same dimensions"
        );

        self.data.iter_mut().zip(v.data.iter()).for_each(|(a, b)| {
            *a = *a - *b;
        });
        self
    }

    pub fn scl(&mut self, a: T) -> &Self
    where
        T: Mul<Output = T>,
    {
        self.data.iter_mut().for_each(|x| {
            *x = *x * a;
        });
        self
    }

    pub fn dot(&self, v: Vector<T>) -> T
    where
        T: Mul<Output = T> + Add<Output = T> + Sum<T>,
    {
        assert_eq!(
            self.data.len(),
            v.data.len(),
            "Vectors must have the same dimensions"
        );
        self.data
            .iter()
            .zip(v.data.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }
}

impl<T> Vector<T>
where
    T: Copy + Into<f32>,
{
    pub fn norm_1(&self) -> f32 {
        self.data.iter().map(|&x| Into::<f32>::into(x).abs()).sum()
    }
    pub fn norm(&self) -> f32 {
        self.data
            .iter()
            .map(|&x| {
                let f: f32 = x.into();
                f * f
            })
            .sum::<f32>()
            .sqrt()
    }
    pub fn norm_inf(&self) -> f32 {
        self.data
            .iter()
            .map(|&x| Into::<f32>::into(x).abs())
            .fold(f32::NEG_INFINITY, f32::max)
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
