#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>(pub [[T; W]; H]);

impl<const W: usize, const H: usize, T:Copy + Default> Matrix<W, H, T> {
    pub fn zero() -> Self {
        Self([[T::default(); W];H])
    }
}

impl<const S: usize, T:Copy+Default+From<u8>> Matrix<S, S, T> {
    pub fn identity() -> Self {
        let mut m = [[T::default(); S]; S];
        for i in 0..S{
            m[i][i] = T::from(1u8);
        }
        Self(m)
    }
}