use crate::float::F64IsAbout;

pub type Matrix2 = [[f64; 2]; 2];

pub trait Matrix2Methods {
    fn equals(&self, peer: &Matrix2) -> bool;
    fn determinant(&self) -> f64;
}

impl Matrix2Methods for Matrix2 {

    fn equals(self: &Matrix2, peer: &Matrix2) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !self[i][j].equals(peer[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    fn determinant(self: &Matrix2) -> f64 {
        return self[0][0] * self[1][1] - self[1][0] * self[0][1];
    }

}

pub fn matrix2() -> Matrix2 {
    let matrix: Matrix2 = [[0.0; 2]; 2];
    return matrix;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_create() {
        let m: Matrix2 = [[-3.0, 5.0], [1.0, -2.0]];
        assert!(m[0][0].equals(-3.0));
        assert!(m[0][1].equals(5.0));
        assert!(m[1][0].equals(1.0));
        assert!(m[1][1].equals(-2.0));
    }

    #[test]
    fn should_calculate_determinant() {
        let a: Matrix2 = [[1.0, 5.0], [-3.0, 2.0]];
        assert!(a.determinant().equals(17.0));
    }
}
