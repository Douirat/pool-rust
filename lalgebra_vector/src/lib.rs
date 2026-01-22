use std::ops::Add;

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
