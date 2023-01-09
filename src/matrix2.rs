use crate::float::Float;
use std::cmp::{PartialEq};
use std::ops::{Index, IndexMut};

pub struct Matrix2([[Float; 2]; 2]);

impl Matrix2 {
    pub fn new() -> Matrix2 {
        return Matrix2([[Float::from(0);  2]; 2]);
    }

    pub fn from<S: Into<f64> + Copy>(values: [[S; 2]; 2]) -> Matrix2 {
        let mut m = Matrix2::new();
        for i in 0..2 {
            for j in 0..2 {
                m[[i, j]] = Float::from(values[i][j].into());
            }
        }
        return m;
    }

    pub fn determinant(self: &Matrix2) -> Float {
        return self[[0,0]] * self[[1,1]] - self[[1,0]] * self[[0,1]];
    }
}

impl PartialEq<Matrix2> for Matrix2 {
    fn eq(&self, other: &Matrix2) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if self[[i, j]] != other[[i, j]] {
                    return false;
                }
            }
        }
        return true;
    }


    fn ne(&self, other: &Matrix2) -> bool {
        return !self.eq(other);
    }
}


impl std::ops::Index<[usize; 2]> for Matrix2 {
    type Output = Float;

    fn index(&self, idx: [usize; 2]) -> &Float {
        return &self.0[idx[0]][idx[1]];
    }
}

impl std::ops::IndexMut<[usize; 2]> for Matrix2 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Float {
        return &mut self.0[idx[0]][idx[1]];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m = Matrix2::from([[-3, 5], [1, -2]]);
        assert!(m[[0, 0]] == -3);
        assert!(m[[0, 1]] == 5);
        assert!(m[[1, 0]] == 1);
        assert!(m[[1, 1]] == -2);
    }

    #[test]
    fn should_calculate_determinant() {
        let a = Matrix2::from([[1, 5], [-3, 2]]);
        assert!(a.determinant() == 17.0);
    }
}
