matrix_opsuse std::ops::{Add, AddAssign, Sub, Mul};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Matrix<const W: usize, const H: usize, T>( [[T; W]; H]);

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Wrapper<const W:usize,const H: usize, T>( Matrix<W, H, T>);

impl<const W: usize, const H: usize, T> From<[[T; W]; H]> for Wrapper<W, H, T> {
 fn from(arr: [[T; W]; H]) -> Self {
    Wrapper(Matrix(arr))
}
}

impl<const W: usize, const H: usize, T> Add for Wrapper<W, H, T> where T: Add<Output = T> + Copy {
type Output = Wrapper<W, H, T>;
 fn add(self, other: Self)-> Self::Output{
    let mut result = self.0.0;
    for i in 0..H{
        for j in 0..W {
            result[i][j] = self.0.0[i][j] + other.0.0[i][j];
        }
    }
     Self(Matrix(result))
}
}

impl<const W: usize, const H: usize, T> Sub for Wrapper<W, H, T> where T: Sub<Output=T> + Copy {
type Output = Wrapper<W, H, T>;
 fn sub(self, other: Self) -> Self::Output {
    let mut result = self.0.0;
        for i in 0..H{
        for j in 0..W{
            result[i][j] = self.0.0[i][j] - other.0.0[i][j];
        }
    }
     Self(Matrix(result))
} 
}

impl<const S: usize, T> Mul for Wrapper<S, S, T> where T: Mul<Output=T> + Add<Output = T> + Copy + Default{
type Output = Wrapper<S, S, T>;
 fn mul(self, other: Self) -> Self::Output {
           let mut result = [[T::default(); S]; S];

        for i in 0..S {
            for j in 0..S {
                for k in 0..S {
                    result[i][j] =
                        result[i][j] + self.0.0[i][k] * other.0.0[k][j];
                }
            }
        }

        Wrapper(Matrix(result))
} 
}