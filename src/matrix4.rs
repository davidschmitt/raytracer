use std::convert::AsRef;
use std::cmp::PartialEq;
use std::ops::{Index, IndexMut, Mul};
use crate::float::Float;
use crate::matrix3::Matrix3;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub struct Matrix4([[Float; 4]; 4]);

impl AsRef<Matrix4> for Matrix4 {
    fn as_ref(&self) -> &Matrix4 {
        return self;
    }
}

impl PartialEq<Matrix4> for Matrix4 {
    fn eq(&self, other: &Matrix4) -> bool {
        let peer = other.as_ref();
        for i in 0..4 {
            for j in 0..4 {
                if self[[i, j]] != peer[[i, j]] {
                    return false;
                }
            }
        }
        return true;
    }
    fn ne(&self, other: &Matrix4) -> bool {
        return !self.eq(other);
    }
}

impl <S: AsRef<Matrix4>> Mul<S> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, rhs: S) -> Self::Output {
        let mut m = Matrix4::new();
        let a = &self.0;
        let b = &rhs.as_ref().0;

        for i in 0..4 {
            for j in 0..4 {
                m.0[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
            }
        }

        return m;
    }
}

impl Mul<Tuple> for &Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}

impl Mul<Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}

impl Mul<&Tuple> for &Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}

impl Mul<&Tuple> for Matrix4 {
    type Output = Tuple;
    fn mul(self, rhs: &Tuple) -> Self::Output {
        return self.times_tuple(rhs);
    }
}

impl Index<[usize; 2]> for Matrix4 {
    type Output = Float;

    fn index(&self, idx: [usize; 2]) -> &Float {
        return &self.0[idx[0]][idx[1]];
    }
}

impl IndexMut<[usize; 2]> for Matrix4 {
    fn index_mut(&mut self, idx: [usize; 2]) -> &mut Float {
        return &mut self.0[idx[0]][idx[1]];
    }
}

impl Matrix4 {

    pub fn new() -> Matrix4 {
        return Matrix4([[Float::from(0); 4]; 4]);
    }

    pub fn from<S: Into<f64> + Copy>(values: [[S; 4]; 4]) -> Matrix4 {
        let mut m = Matrix4::new();
        for i in 0..4  {
            for j in 0..4 {
                m[[i, j]] = Float::from(values[i][j].into());
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
        return Tuple::new(x, y, z, w);
    }

    pub fn transpose(self: &Matrix4) -> Matrix4 {
        let m = self.0;
        return Matrix4([
            [m[0][0], m[1][0], m[2][0], m[3][0]],
            [m[0][1], m[1][1], m[2][1], m[3][1]],
            [m[0][2], m[1][2], m[2][2], m[3][2]],
            [m[0][3], m[1][3], m[2][3], m[3][3]],
        ]);
    }

    pub fn submatrix(self: &Matrix4, x: usize, y: usize) -> Matrix3 {
        let m = self.0;
        let mut m3 = Matrix3::new();
        let mut w = 0;
        for i in 0..4 {
            if i != x {
                let mut v = 0;
                for j in 0..4 {
                    if j != y {
                        m3[[w, v]] = m[i][j];
                        v += 1;
                    }
                }
                w += 1
            }
        }
        return m3;
    }

    pub fn minor(self: &Matrix4, x: usize, y: usize) -> Float {
        let m3 = self.submatrix(x, y);
        return m3.determinant();
    }

    pub fn cofactor(self: &Matrix4, x: usize, y: usize) -> Float {
        if (x + y) % 2 == 0 {
            return self.minor(x, y);
        } else {
            return -self.minor(x, y);
        }
    }

    pub fn determinant(self: &Matrix4) -> Float {
        let m = self.0;
        return self.cofactor(0, 0) * m[0][0]
            + self.cofactor(1, 0) * m[1][0]
            + self.cofactor(2, 0) * m[2][0]
            + self.cofactor(3, 0) * m[3][0];
    }

    pub fn is_invertible(self: &Matrix4) -> bool {
        return self.determinant() != 0;
    }

    pub fn inverse(self: &Matrix4) -> Matrix4 {
        assert!(self.is_invertible());
        let mut m2 = Matrix4::new();
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
    use crate::transform::Transform;
    use super::*;

    //  Page 26
    #[test]
    fn should_create() {
        let m = Matrix4::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);
        assert!(m[[0, 0]] == 1.0);
        assert!(m[[0, 3]] == 4.0);
        assert!(m[[1, 0]] == 5.5);
        assert!(m[[1, 2]] == 7.5);
        assert!(m[[2, 2]] == 11.0);
        assert!(m[[3, 0]] == 13.5);
        assert!(m[[3, 2]] == 15.5);
    }

    #[test]
    fn should_check_equality() {
        let a = Matrix4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 8, 7, 6],
            [5, 4, 3, 2],
        ]);
        let b = Matrix4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 8, 7, 6],
            [5, 4, 3, 2],
        ]);
        assert!(a == b);
        let c = Matrix4::new();
        let d = Matrix4::new();
        assert!(c == d);
    }

    #[test]
    fn should_check_inequality() {
        let a = Matrix4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 8, 7, 6],
            [5, 4, 3, 2],
        ]);
        let b = Matrix4::from([
            [2, 3, 4, 5],
            [6, 7, 8, 9],
            [8, 7, 6, 5],
            [4, 3, 2, 1],
        ]);
        assert!(a != b);
        let c = Matrix4::new();
        let mut d = Matrix4::new();
        d.0[0][0] = Float::from(1.0);
        assert!(c != d);
    }

    #[test]
    fn should_multiply() {
        let a = Matrix4::from([
            [1, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 8, 7, 6],
            [5, 4, 3, 2],
        ]);
        let b = Matrix4::from([
            [-2, 1, 2, 3],
            [3, 2, 1, -1],
            [4, 3, 6, 5],
            [1, 2, 7, 8],
        ]);
        let expected = Matrix4::from([
            [20, 22, 50, 48],
            [44, 54, 114, 108],
            [40, 58, 110, 102],
            [16, 26, 46, 42],
        ]);
        let result = a * b;
        assert!(result == expected);
    }

    #[test]
    fn should_multiply_tuple() {
        let m= Matrix4::from([
            [1, 2, 3, 4],
            [2, 4, 4, 2],
            [8, 6, 4, 1],
            [0, 0, 0, 1],
        ]);
        let t = Tuple::new(1, 2, 3, 1);
        let expected = Tuple::new(18, 24, 33, 1);
        let result = m * t;
        assert!(expected == result);
    }

    #[test]
    fn should_multiply_identity_matrix() {
        let a = Matrix4::from([
            [0, 1, 2, 4],
            [1, 2, 4, 8],
            [2, 4, 8, 16],
            [4, 8, 16, 32],
        ]);
        let result = a * Transform::identity();
        assert!(result == a);

        let t = Tuple::new(1, 2, 3, 4);
        let tresult = Transform::identity() * t;
        assert!(t == tresult);
    }

    #[test]
    fn should_transpose() {
        let a = Matrix4::from([
            [0, 9, 3, 0],
            [9, 8, 0, 8],
            [1, 8, 5, 3],
            [0, 0, 5, 8],
        ]);
        let expected = Matrix4::from([
            [0, 9, 1, 0],
            [9, 8, 8, 0],
            [3, 0, 5, 5],
            [0, 8, 3, 8],
        ]);
        let result = a.transpose();
        assert!(result == expected);

        let iresult = Transform::identity().transpose();
        assert!(iresult == Transform::identity());
    }

    #[test]
    fn should_calculate_submatrix() {
        let a = Matrix4::from([
            [-6, 1, 1, 6],
            [-8, 5, 8, 6],
            [-1, 0, 8, 2],
            [-7, 1, -1, 1],
        ]);
        let result = a.submatrix(2, 1);
        let expected = Matrix3::from([[-6, 1, 6], [-8, 8, 6], [-7, -1, 1]]);
        assert!(result == expected);
    }

    #[test]
    fn should_calculate_determinant() {
        let a = Matrix4::from([
            [-2, -8, 3, 5],
            [-3, 1, 7, 3],
            [1, 2, -9, 6],
            [-6, 7, 7, -9],
        ]);
        assert!(a.cofactor(0, 0) == 690);
        assert!(a.cofactor(0, 1) == 447);
        assert!(a.cofactor(0, 2) == 210);
        assert!(a.cofactor(0, 3) == 51);
        assert!(a.determinant() == -4071);
    }

    #[test]
    fn should_calculate_invertibility() {
        let a = Matrix4::from([
            [6, 4, 4, 4],
            [5, 5, 7, 6],
            [4, -9, 3, -7],
            [9, 1, 7, -6],
        ]);
        assert!(a.determinant() == -2120);
        assert!(a.is_invertible());

        let a = Matrix4::from([
            [-4, 2, -2, -3],
            [9, 6, 2, 6],
            [0, -5, 1, -5],
            [0, 0, 0, 0],
        ]);
        assert!(a.determinant() == 0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn should_invert() {
        let a = Matrix4::from([
            [-5, 2, 6, -8],
            [1, -5, 1, 8],
            [7, 7, -6, -7],
            [1, -3, 7, 4],
        ]);
        let b = a.inverse();
        let expected = Matrix4::from([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);

        assert!(a.determinant() == 532);
        assert!(a.cofactor(2, 3) == -160);
        assert!(b.0[3][2] == -160.0 / 532.0);
        assert!(a.cofactor(3, 2) == 105);
        assert!(b.0[2][3] == 105.0 / 532.0);
        println!("{:?}", &b);
        println!("{:?}", &expected);
        assert!(b == expected);
    }

    #[test]
    fn should_invert_more() {
        let a = Matrix4::from([
            [8, -5, 9, 2],
            [7, 5, 6, 1],
            [-6, 0, 9, 6],
            [-3, 0, -9, -4],
        ]);
        let result = a.inverse();
        let expected = Matrix4::from([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        assert!(result == expected);
    }

    #[test]
    fn should_invert_even_more() {
        let a = Matrix4::from([
            [9, 3, 0, 9],
            [-5, -2, -6, -3],
            [-4, 9, 6, 4],
            [-7, 6, 6, 2],
        ]);
        let result = a.inverse();
        let expected = Matrix4::from([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);
        assert!(result == expected);
    }

    #[test]
    fn should_can_reverse() {
        let a = Matrix4::from([
            [3, -9, 7, 3],
            [3, -8, 2, -9],
            [-4, 4, 4, 1],
            [-6, 5, -1, 1],
        ]);
        let b = Matrix4::from([
            [8, 2, 2, 2],
            [3, -1, 7, 0],
            [7, 0, 5, 4],
            [6, -2, 0, 5],
        ]);
        let c = a * b;
        let result = c * b.inverse();
        assert!(result == a);
    }
}