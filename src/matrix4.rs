use std::convert::AsRef;
use std::cmp::PartialEq;
use std::ops::{Mul};
use crate::float::F64IsAbout;
use crate::matrix3::{matrix3, Matrix3, Matrix3Methods};
use crate::tuple::{tuple, Tuple};

#[derive(Clone, Copy, Debug)]
pub struct Matrix4(pub [[f64; 4]; 4]);

pub const IDENTITY: Matrix4 = Matrix4 ([
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
]);

impl AsRef<Matrix4> for Matrix4 {
    fn as_ref(&self) -> &Matrix4 {
        return self;
    }
}

impl PartialEq<Matrix4> for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        return self.equals(other);
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: Matrix4) -> Self::Output {
        return self.multiply(rhs);
    }
}

impl Mul<&Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: &Matrix4) -> Self::Output {
        return self.multiply(rhs);
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}

impl Mul<&Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}


pub trait Matrix4Methods {
    fn equals<T: AsRef<Matrix4>>(&self, peer: T) -> bool;
    fn multiply<T: AsRef<Matrix4>>(&self, peer: T) -> Matrix4;
    fn times_tuple<T: AsRef<Tuple>>(&self, tuple: T) -> Tuple;
    fn transpose(&self) -> Matrix4;
    fn submatrix(&self, x: usize, y: usize) -> Matrix3;
    fn minor(&self, x: usize, y: usize) -> f64;
    fn cofactor(&self, x: usize, y: usize) -> f64;
    fn determinant(&self) -> f64;
    fn is_invertible(&self) -> bool;
    fn inverse(&self) -> Matrix4;
}

pub fn matrix4() -> Matrix4 {
    let matrix = Matrix4 ([[0.0; 4]; 4]);
    return matrix;
}

impl Matrix4Methods for Matrix4 {
    fn equals<T: AsRef<Matrix4>>(self: &Matrix4, other: T) -> bool {
        let peer = other.as_ref();
        for i in 0..4 {
            for j in 0..4 {
                if !self.0[i][j].equals(peer.0[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    fn multiply<T: AsRef<Matrix4>>(self: &Matrix4, other: T) -> Matrix4 {
        let mut m = matrix4();
        let a = &self.0;
        let b = &other.as_ref().0;

        for i in 0..4 {
            for j in 0..4 {
                m.0[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
            }
        }

        return m;
    }

    fn times_tuple<T: AsRef<Tuple>>(self: &Matrix4, other: T) -> Tuple {
        let a = &self.0;
        let t = other.as_ref();
        let x = a[0][0] * t.x + a[0][1] * t.y + a[0][2] * t.z + a[0][3] * t.w;
        let y = a[1][0] * t.x + a[1][1] * t.y + a[1][2] * t.z + a[1][3] * t.w;
        let z = a[2][0] * t.x + a[2][1] * t.y + a[2][2] * t.z + a[2][3] * t.w;
        let w = a[3][0] * t.x + a[3][1] * t.y + a[3][2] * t.z + a[3][3] * t.w;
        return tuple(x, y, z, w);
    }

    fn transpose(self: &Matrix4) -> Matrix4 {
        let m = self.0;
        return Matrix4 ([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ]);
    }

    fn submatrix(self: &Matrix4, x: usize, y: usize) -> Matrix3 {
        let m = self.0;
        let mut m3 = matrix3();
        let mut w = 0;
        for i in 0..4 {
            if i != x {
                let mut v = 0;
                for j in 0..4 {
                    if j != y {
                        m3[w][v] = m[i][j];
                        v += 1;
                    }
                }
                w += 1
            }
        }
        return m3;
    }

    fn minor(self: &Matrix4, x: usize, y: usize) -> f64 {
        let m3 = self.submatrix(x, y);
        return m3.determinant();
    }

    fn cofactor(self: &Matrix4, x: usize, y: usize) -> f64 {
        if (x + y) % 2 == 0 {
            return self.minor(x, y);
        } else {
            return -self.minor(x, y);
        }
    }

    fn determinant(self: &Matrix4) -> f64 {
        let m = self.0;
        return self.cofactor(0, 0) * m[0][0]
            + self.cofactor(1, 0) * m[1][0]
            + self.cofactor(2, 0) * m[2][0]
            + self.cofactor(3, 0) * m[3][0];
    }

    fn is_invertible(self: &Matrix4) -> bool {
        return !self.determinant().equals(0.0)
    }

    fn inverse(self: &Matrix4) -> Matrix4 {
        assert!(self.is_invertible());
        let mut m2 = matrix4();
        let det = self.determinant();
        for i in 0..4 {
            for j in 0..4 {
                m2.0[i][j] = self.cofactor(j, i) / det
            }
        }
        return m2;
    }

}

#[cfg(test)]
mod tests {
    use crate::tuple::{TupleMethods};
    use super::*;

    //  Page 26
    #[test]
    fn should_create() {
        let m = Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert!(m.0[0][0].equals(1.0));
        assert!(m.0[0][3].equals(4.0));
        assert!(m.0[1][0].equals(5.5));
        assert!(m.0[1][2].equals(7.5));
        assert!(m.0[2][2].equals(11.0));
        assert!(m.0[3][0].equals(13.5));
        assert!(m.0[3][2].equals(15.5));
    }

    #[test]
    fn should_check_equality() {
        let a = Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert!(a.equals(&b));
        let c = matrix4();
        let d = matrix4();
        assert!(c.equals(&d));
    }

    #[test]
    fn should_check_inequality() {
        let a = Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4 ([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert!(!a.equals(&b));
        let c = matrix4();
        let mut d = matrix4();
        d.0[0][0] = 1.0;
        assert!(!c.equals(&d));
    }

    #[test]
    fn should_multiply() {
        let a = Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix4 ([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let expected = Matrix4 ([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        let result = a.multiply(&b);
        assert!(result.equals(&expected));
    }

    #[test]
    fn should_multiply_tuple() {
        let m= Matrix4 ([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
        let t = tuple(1.0, 2.0, 3.0, 1.0);
        let expected = tuple(18.0, 24.0, 33.0, 1.0);
        let result = m.times_tuple(&t);
        assert!(expected.equals(&result));
    }

    #[test]
    fn should_multiply_identity_matrix() {
        let a = Matrix4 ([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let result = a.multiply(&IDENTITY);
        assert!(result.equals(&a));

        let t: Tuple = tuple(1.0, 2.0, 3.0, 4.0);
        let tresult = IDENTITY.times_tuple(&t);
        assert!(t.equals(&tresult));
    }

    #[test]
    fn should_transpose() {
        let a = Matrix4 ([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let expected = Matrix4 ([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        let result = a.transpose();
        assert!(result.equals(&expected));

        let iresult = IDENTITY.transpose();
        assert!(&iresult.equals(&IDENTITY));
    }

    #[test]
    fn should_calculate_submatrix() {
        let a = Matrix4 ([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let result = a.submatrix(2, 1);
        let expected: Matrix3 = [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]];
        assert!(result.equals(&expected));
    }

    #[test]
    fn should_calculate_determinant() {
        let a = Matrix4 ([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert!(a.cofactor(0, 0).equals(690.0));
        assert!(a.cofactor(0, 1).equals(447.0));
        assert!(a.cofactor(0, 2).equals(210.0));
        assert!(a.cofactor(0, 3).equals(51.0));
        assert!(a.determinant().equals(-4071.0));
    }

    #[test]
    fn should_calculate_invertibility() {
        let a = Matrix4 ([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        assert!(a.determinant().equals(-2120.0));
        assert!(a.is_invertible());

        let a = Matrix4 ([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert!(a.determinant().equals(0.0));
        assert!(!a.is_invertible());
    }

    #[test]
    fn should_invert() {
        let a = Matrix4 ([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);
        let b = a.inverse();
        let expected = Matrix4 ([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert!(a.determinant().equals(532.0));
        assert!(a.cofactor(2, 3).equals(-160.0));
        assert!(b.0[3][2].equals(-160.0 / 532.0));
        assert!(a.cofactor(3, 2).equals(105.0));
        assert!(b.0[2][3].equals(105.0 / 532.0));
        println!("{:?}", &b);
        println!("{:?}", &expected);
        assert!(b.equals(&expected));
    }

    #[test]
    fn should_invert_more() {
        let a = Matrix4 ([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let result = a.inverse();
        let expected = Matrix4 ([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert!(result.equals(&expected));
    }

    #[test]
    fn should_invert_even_more() {
        let a = Matrix4 ([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let result = a.inverse();
        let expected = Matrix4 ([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert!(result.equals(&expected));
    }

    #[test]
    fn should_can_reverse() {
        let a = Matrix4 ([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix4 ([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let c = a.multiply(&b);
        let result = c.multiply(&b.inverse());
        assert!(result.equals(&a));
    }
}
