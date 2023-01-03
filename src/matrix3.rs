use crate::float::F64IsAbout;
use crate::matrix2::{matrix2, Matrix2, Matrix2Methods};

pub type Matrix3 = [[f64; 3]; 3];

pub trait Matrix3Methods {
    fn equals(&self, peer: &Matrix3) -> bool;
    fn submatrix(&self, x: usize, y: usize) -> Matrix2;
    fn minor(&self, x: usize, y: usize) -> f64;
    fn cofactor(&self, x: usize, y: usize) -> f64;
    fn determinant(&self) -> f64;
}

impl Matrix3Methods for Matrix3 {
    fn equals(self: &Matrix3, peer: &Matrix3) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !self[i][j].is_about(peer[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    fn submatrix(self: &Matrix3, x: usize, y: usize) -> Matrix2 {
        let mut m2 = matrix2();
        let mut w = 0;
        for i in 0..3 {
            if i != x {
                let mut v = 0;
                for j in 0..3 {
                    if j != y {
                        m2[w][v] = self[i][j];
                        v += 1;
                    }
                }
                w += 1
            }
        }
        return m2;
    }

    fn minor(self: &Matrix3, x: usize, y: usize) -> f64 {
        let m2 = self.submatrix(x, y);
        return m2.determinant();
    }

    fn cofactor(self: &Matrix3, x: usize, y: usize) -> f64 {
        if (x + y) % 2 == 0 {
            return self.minor(x, y);
        } else {
            return -self.minor(x, y);
        }
    }

    fn determinant(self: &Matrix3) -> f64 {
        return self.cofactor(0, 0) * self[0][0]
            + self.cofactor(1, 0) * self[1][0]
            + self.cofactor(2, 0) * self[2][0];
    }
}

pub fn matrix3() -> Matrix3 {
    let matrix: Matrix3 = [[0.0; 3]; 3];
    return matrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m: Matrix3 = [[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]];
        assert!(m[0][0].is_about(-3.0));
        assert!(m[0][1].is_about(5.0));
        assert!(m[1][0].is_about(1.0));
        assert!(m[1][1].is_about(-2.0));
    }

    #[test]
    fn should_calculate_submatrix() {
        let a: Matrix3 = [[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]];
        let expected: Matrix2 = [[-3.0, 2.0], [0.0, 6.0]];
        let result = a.submatrix(0, 2);
        assert!(result.equals(&expected));
    }

    #[test]
    fn should_calculate_minor() {
        let a: Matrix3 = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
        let b = a.submatrix(1, 0);
        assert!(b.determinant().is_about(25.0));
        assert!(a.minor(1, 0).is_about(25.0));
    }

    #[test]
    fn should_calculate_cofactor() {
        let a: Matrix3 = [[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]];
        assert!(a.minor(0, 0).is_about(-12.0));
        assert!(a.cofactor(0, 0).is_about(-12.0));
        assert!(a.minor(1, 0).is_about(25.0));
        assert!(a.cofactor(1, 0).is_about(-25.0));
    }

    #[test]
    fn should_calculate_determinant() {
        let a: Matrix3 = [[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]];
        assert!(a.cofactor(0, 0).is_about(56.0));
        assert!(a.cofactor(0, 1).is_about(12.0));
        assert!(a.cofactor(0, 2).is_about(-46.0));
        assert!(a.determinant().is_about(-196.0));
    }
}
