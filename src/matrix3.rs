use crate::float::Float;
use crate::matrix2::Matrix2;
use std::ops::{Index, IndexMut};

pub struct Matrix3([[Float; 3]; 3]);

impl PartialEq<Matrix3> for Matrix3 {
    fn eq(&self, other: &Matrix3) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if self[[i, j]] != other[[i, j]] {
                    return false;
                }
            }
        }
        return true;
    }
    fn ne(&self, other: &Matrix3) -> bool {
        return !self.eq(other);
    }
}

impl Index<[usize; 2]> for Matrix3 {
    type Output = Float;

    fn index(&self, idx: [usize; 2]) -> &Float {
        return &self.0[idx[0]][idx[1]];
    }
}

impl IndexMut<[usize; 2]> for Matrix3 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Float {
        return &mut self.0[idx[0]][idx[1]];
    }
}

impl Matrix3 {
    pub fn new() -> Matrix3 {
        return Matrix3([[Float::from(0); 3]; 3]);
    }

    pub fn from<S: Into<f64> + Copy>(values: [[S; 3]; 3]) -> Matrix3 {
        let mut m = Matrix3::new();
        for i in 0..3 {
            for j in 0..3 {
                m[[i, j]] = Float::from(values[i][j].into());
            }
        }
        return m;
    }

    pub fn submatrix(self: &Matrix3, x: usize, y: usize) -> Matrix2 {
        let mut m2 = Matrix2::new();
        let mut w = 0;
        for i in 0..3 {
            if i != x {
                let mut v = 0;
                for j in 0..3 {
                    if j != y {
                        m2[[w, v]] = self[[i, j]];
                        v += 1;
                    }
                }
                w += 1
            }
        }
        return m2;
    }

    pub fn minor(self: &Matrix3, x: usize, y: usize) -> Float {
        let m2 = self.submatrix(x, y);
        return m2.determinant();
    }

    pub fn cofactor(self: &Matrix3, x: usize, y: usize) -> Float {
        if (x + y) % 2 == 0 {
            return self.minor(x, y);
        } else {
            return -self.minor(x, y);
        }
    }

    pub fn determinant(self: &Matrix3) -> Float {
        return self.cofactor(0, 0) * self[[0, 0]]
            + self.cofactor(1, 0) * self[[1, 0]]
            + self.cofactor(2, 0) * self[[2, 0]];
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m = Matrix3::from([[-3, 5, 0], [1, -2, -7], [0, 1, 1]]);
        assert!(m[[0, 0]] == -3);
        assert!(m[[0, 1]] == 5);
        assert!(m[[1, 0]] == 1);
        assert!(m[[1, 1]] == -2);
    }

    #[test]
    fn should_calculate_submatrix() {
        let a = Matrix3::from([[1, 5, 0], [-3, 2, 7], [0, 6, -3]]);
        let expected = Matrix2::from([[-3, 2], [0, 6]]);
        let result = a.submatrix(0, 2);
        assert!(result == expected);
    }

    #[test]
    fn should_calculate_minor() {
        let a = Matrix3::from([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        let b = a.submatrix(1, 0);
        assert!(b.determinant() == 25);
        assert!(a.minor(1, 0) == 25);
    }

    #[test]
    fn should_calculate_cofactor() {
        let a = Matrix3::from([[3, 5, 0], [2, -1, -7], [6, -1, 5]]);
        assert!(a.minor(0, 0) == -12);
        assert!(a.cofactor(0, 0) == -12);
        assert!(a.minor(1, 0) == 25);
        assert!(a.cofactor(1, 0) == -25);
    }

    #[test]
    fn should_calculate_determinant() {
        let a = Matrix3::from([[1, 2, 6], [-5, 8, -4], [2, 6, 4]]);
        assert!(a.cofactor(0, 0) == 56);
        assert!(a.cofactor(0, 1) == 12);
        assert!(a.cofactor(0, 2) == -46);
        assert!(a.determinant() == -196);
    }
}
