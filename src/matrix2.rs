use crate::float;

pub type Matrix2 = [[f64; 2]; 2];

pub fn matrix2() -> Matrix2 {
    let matrix: Matrix2 = [[0.0; 2]; 2];
    return matrix;
}

pub fn equals(a: &Matrix2, b: &Matrix2) -> bool {
    for i in 0..2 {
        for j in 0..2 {
            if !float::equals(a[i][j], b[i][j]) {
                return false;
            }
        }
    }
    return true;
}

pub fn determinant(m: &Matrix2) -> f64 {
    return m[0][0] * m[1][1] - m[1][0] * m[0][1];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m: Matrix2 = [[-3.0, 5.0], [1.0, -2.0]];
        assert!(float::equals(m[0][0], -3.0));
        assert!(float::equals(m[0][1], 5.0));
        assert!(float::equals(m[1][0], 1.0));
        assert!(float::equals(m[1][1], -2.0));
    }

    #[test]
    fn should_calculate_determinant() {
        let a: Matrix2 = [[1.0, 5.0], [-3.0, 2.0]];
        assert!(float::equals(determinant(&a), 17.0));
    }
}
