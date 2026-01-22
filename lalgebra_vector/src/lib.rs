use std::ops::{Add, Mul};

/// Scalar trait
pub trait Scalar:
    Copy
    + Add<Output = Self>
    + Mul<Output = Self>
    + Default
    + PartialEq
    + std::fmt::Debug
{
}

/// Blanket implementation
impl<T> Scalar for T
where
    T: Copy
        + Add<Output = T>
        + Mul<Output = T>
        + Default
        + PartialEq
        + std::fmt::Debug,
{
}

#[derive(Debug, PartialEq)]
pub struct Vector<T: Scalar>(pub Vec<T>);

impl<T: Scalar> Add for Vector<T> {
    type Output = Option<Vector<T>>;

    fn add(self, rhs: Self) -> Self::Output {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let data = self
            .0
            .into_iter()
            .zip(rhs.0.into_iter())
            .map(|(a, b)| a + b)
            .collect();

        Some(Vector(data))
    }
}

impl<T: Scalar> Vector<T> {
    pub fn dot(self, rhs: Self) -> Option<T> {
        if self.0.len() != rhs.0.len() {
            return None;
        }

        let mut sum = T::default();

        for (a, b) in self.0.into_iter().zip(rhs.0.into_iter()) {
            sum = sum + a * b;
        }

        Some(sum)
    }
}
