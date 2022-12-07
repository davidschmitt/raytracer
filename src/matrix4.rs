use crate::float;
use crate::matrix2;
use crate::matrix3;
use crate::tuple;

pub type Matrix4 = [[f64; 4]; 4];

pub const IDENTITY: Matrix4 = [
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 1.0, 0.0],
    [0.0, 0.0, 0.0, 1.0],
];

pub fn matrix4() -> Matrix4 {
    let matrix: Matrix4 = [[0.0; 4]; 4];
    return matrix;
}

pub fn equals(a: &Matrix4, b: &Matrix4) -> bool {
    for i in 0..4 {
        for j in 0..4 {
            if !float::equals(a[i][j], b[i][j]) {
                return false;
            }
        }
    }
    return true;
}

pub fn multiply(a: &Matrix4, b: &Matrix4) -> Matrix4 {
    let mut m = matrix4();

    for i in 0..4 {
        for j in 0..4 {
            m[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j] + a[i][2] * b[2][j] + a[i][3] * b[3][j];
        }
    }

    return m;
}

pub fn multiply_tuple(a: &Matrix4, b: &tuple::Tuple) -> tuple::Tuple {
    let x = a[0][0] * b.x + a[0][1] * b.y + a[0][2] * b.z + a[0][3] * b.w;
    let y = a[1][0] * b.x + a[1][1] * b.y + a[1][2] * b.z + a[1][3] * b.w;
    let z = a[2][0] * b.x + a[2][1] * b.y + a[2][2] * b.z + a[2][3] * b.w;
    let w = a[3][0] * b.x + a[3][1] * b.y + a[3][2] * b.z + a[3][3] * b.w;
    return tuple::tuple(x, y, z, w);
}

pub fn transpose(m: &Matrix4) -> Matrix4 {
    return [
        [m[0][0], m[1][0], m[2][0], m[3][0]],
        [m[0][1], m[1][1], m[2][1], m[3][1]],
        [m[0][2], m[1][2], m[2][2], m[3][2]],
        [m[0][3], m[1][3], m[2][3], m[3][3]],
    ];
}

pub fn submatrix(m4: &Matrix4, x: usize, y: usize) -> matrix3::Matrix3 {
    let mut m3 = matrix3::matrix3();
    let mut w = 0;
    for i in 0..4 {
        if i != x {
            let mut v = 0;
            for j in 0..4 {
                if j != y {
                    m3[w][v] = m4[i][j];
                    v += 1;
                }
            }
            w += 1
        }
    }
    return m3;
}

pub fn minor(m4: &Matrix4, x: usize, y: usize) -> f64 {
    let m3 = submatrix(m4, x, y);
    return matrix3::determinant(&m3);
}

pub fn cofactor(m4: &Matrix4, x: usize, y: usize) -> f64 {
    if (x + y) % 2 == 0 {
        return minor(m4, x, y);
    } else {
        return -minor(m4, x, y);
    }
}

pub fn determinant(m4: &Matrix4) -> f64 {
    return cofactor(m4, 0, 0) * m4[0][0]
        + cofactor(m4, 1, 0) * m4[1][0]
        + cofactor(m4, 2, 0) * m4[2][0]
        + cofactor(m4, 3, 0) * m4[3][0];
}

pub fn is_invertible(m4: &Matrix4) -> bool {
    if float::equals(determinant(m4), 0.0) {
        return false;
    } else {
        return true;
    }
}

pub fn inverse(m: &Matrix4) -> Matrix4 {
    assert!(is_invertible(m));
    let mut m2 = matrix4();
    let det = determinant(m);
    for i in 0..4 {
        for j in 0..4 {
            m2[i][j] = cofactor(m, j, i) / det
        }
    }
    return m2;
}

#[cfg(test)]
mod tests {
    use super::*;

    //  Page 26
    #[test]
    fn should_create() {
        let m: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ];
        assert!(float::equals(m[0][0], 1.0));
        assert!(float::equals(m[0][3], 4.0));
        assert!(float::equals(m[1][0], 5.5));
        assert!(float::equals(m[1][2], 7.5));
        assert!(float::equals(m[2][2], 11.0));
        assert!(float::equals(m[3][0], 13.5));
        assert!(float::equals(m[3][2], 15.5));
    }

    #[test]
    fn should_check_equality() {
        let a: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        assert!(equals(&a, &b));
        let c = matrix4();
        let d = matrix4();
        assert!(equals(&c, &d));
    }

    #[test]
    fn should_check_inequality() {
        let a: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b: Matrix4 = [
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ];
        assert!(!equals(&a, &b));
        let c = matrix4();
        let mut d = matrix4();
        d[0][0] = 1.0;
        assert!(!equals(&c, &d));
    }

    #[test]
    fn should_multiply() {
        let a: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ];
        let b: Matrix4 = [
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ];
        let expected: Matrix4 = [
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ];
        let result = multiply(&a, &b);
        assert!(equals(&result, &expected));
    }

    #[test]
    fn should_multiplyTuple() {
        let m: Matrix4 = [
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let t = tuple::tuple(1.0, 2.0, 3.0, 1.0);
        let expected = tuple::tuple(18.0, 24.0, 33.0, 1.0);
        let result = multiply_tuple(&m, &t);
        assert!(tuple::equals(&expected, &result));
    }

    #[test]
    fn should_multiply_identity_matrix() {
        let a: Matrix4 = [
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ];
        let result = multiply(&a, &IDENTITY);
        assert!(equals(&result, &a));

        let t: tuple::Tuple = tuple::tuple(1.0, 2.0, 3.0, 4.0);
        let tresult = multiply_tuple(&IDENTITY, &t);
        assert!(tuple::equals(&t, &tresult));
    }

    #[test]
    fn should_transpose() {
        let a: Matrix4 = [
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ];
        let expected: Matrix4 = [
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ];
        let result = transpose(&a);
        assert!(equals(&result, &expected));

        let iresult = transpose(&IDENTITY);
        assert!(equals(&iresult, &IDENTITY));
    }

    #[test]
    fn should_calculate_submatrix() {
        let a: Matrix4 = [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];
        let result = submatrix(&a, 2, 1);
        let expected: matrix3::Matrix3 = [[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]];
        assert!(matrix3::equals(&result, &expected));
    }

    #[test]
    fn should_calculate_determinant() {
        let a: Matrix4 = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ];
        assert!(float::equals(cofactor(&a, 0, 0), 690.0));
        assert!(float::equals(cofactor(&a, 0, 1), 447.0));
        assert!(float::equals(cofactor(&a, 0, 2), 210.0));
        assert!(float::equals(cofactor(&a, 0, 3), 51.0));
        assert!(float::equals(determinant(&a), -4071.0));
    }

    #[test]
    fn should_calculate_invertibility() {
        let a: Matrix4 = [
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ];
        assert!(float::equals(determinant(&a), -2120.0));
        assert!(is_invertible(&a));

        let a: Matrix4 = [
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        assert!(float::equals(determinant(&a), 0.0));
        assert!(!is_invertible(&a));
    }

    #[test]
    fn should_invert() {
        let a: Matrix4 = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ];
        let b = inverse(&a);
        let expected: Matrix4 = [
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ];

        assert!(float::equals(determinant(&a), 532.0));
        assert!(float::equals(cofactor(&a, 2, 3), -160.0));
        assert!(float::equals(b[3][2], -160.0 / 532.0));
        assert!(float::equals(cofactor(&a, 3, 2), 105.0));
        assert!(float::equals(b[2][3], 105.0 / 532.0));
        println!("{:?}", &b);
        println!("{:?}", &expected);
        assert!(equals(&b, &expected));
    }

    #[test]
    fn should_invert_more() {
        let a: Matrix4 = [
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ];
        let result = inverse(&a);
        let expected: Matrix4 = [
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ];
        assert!(equals(&result, &expected));
    }

    #[test]
    fn should_invert_even_more() {
        let a: Matrix4 = [
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ];
        let result = inverse(&a);
        let expected: Matrix4 = [
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ];
        assert!(equals(&result, &expected));
    }

    #[test]
    fn should_can_reverse() {
        let a: Matrix4 = [
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ];
        let b: Matrix4 = [
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ];
        let c = multiply(&a, &b);
        let result = multiply(&c, &inverse(&b));
        assert!(equals(&result, &a));
    }
}
