use crate::float;
use crate::matrix2;

pub type Matrix3 = [[f64; 3]; 3];

pub fn matrix3() -> Matrix3 {
    let matrix: Matrix3 = [[0.0; 3]; 3];
    return matrix;
}

pub fn equals(a: &Matrix3, b: &Matrix3) -> bool{
    for i in 0..3 {
        for j in 0..3 {
            if !float::equals(a[i][j], b[i][j]) {
                return false;
            }
        }
    }
    return true;
}

pub fn submatrix(m3: &Matrix3, x: usize, y: usize) -> matrix2::Matrix2 {
    let mut m2 = matrix2::matrix2();
    let mut w = 0;
    for i in 0..3 {
        if i != x {
            let mut v = 0;
            for j in 0..3 {
                if j != y {
                    m2[w][v] = m3[i][j];
                    v += 1;
                }
            }
            w += 1
        }
    }
    return m2;
}

pub fn minor(m3: &Matrix3, x: usize, y: usize) -> f64 {
    let m2 = submatrix(m3, x, y);
    return matrix2::determinant(&m2);
}

pub fn cofactor(m3: &Matrix3, x: usize, y: usize) -> f64{
    if (x + y)%2 == 0 {
        return minor(m3, x, y);
    } else {
        return -minor(m3, x, y);
    }
}

pub fn determinant(m3: &Matrix3) -> f64 {
    return cofactor(m3, 0, 0) * m3[0][0] +
    cofactor(m3, 1, 0) * m3[1][0] +
    cofactor(m3, 2, 0) * m3[2][0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m: Matrix3 =[
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        ];
        assert!(float::equals(m[0][0], -3.0));
        assert!(float::equals(m[0][1], 5.0));
        assert!(float::equals(m[1][0], 1.0));
        assert!(float::equals(m[1][1], -2.0));
    }

    #[test]
    fn should_calculate_submatrix() {
        let a: Matrix3 = [
            [1.0, 5.0, 0.0],
            [-3.0, 2.0, 7.0],
            [0.0, 6.0, -3.0],
        ];
        let expected: matrix2::Matrix2 = [
            [-3.0, 2.0],
            [0.0, 6.0]
        ];
        let result = submatrix(&a, 0, 2);
        assert!(matrix2::equals(&result, &expected));
    }

    #[test]
    fn should_calculate_minor() {
        let a: Matrix3 = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ];
        let b = submatrix(&a, 1, 0);
        assert!(float::equals(matrix2::determinant(&b), 25.0));
        assert!(float::equals(minor(&a, 1, 0), 25.0));
    }

    #[test]
    fn should_calculate_cofactor() {
        let a: Matrix3 = [
            [3.0, 5.0, 0.0],
            [2.0, -1.0, -7.0],
            [6.0, -1.0, 5.0]
        ];
        assert!(float::equals(minor(&a, 0, 0), -12.0));
        assert!(float::equals(cofactor(&a, 0, 0), -12.0));
        assert!(float::equals(minor(&a, 1, 0), 25.0));
        assert!(float::equals(cofactor(&a, 1, 0), -25.0));
    }

    #[test]
    fn should_calculate_determinant() {
        let a: Matrix3 = [
            [1.0, 2.0, 6.0],
            [-5.0, 8.0, -4.0],
            [2.0, 6.0, 4.0]
        ];
        assert!(float::equals(cofactor(&a, 0, 0), 56.0));
        assert!(float::equals(cofactor(&a, 0, 1), 12.0));
        assert!(float::equals(cofactor(&a, 0, 2), -46.0));
        assert!(float::equals(determinant(&a), -196.0));
    }
}