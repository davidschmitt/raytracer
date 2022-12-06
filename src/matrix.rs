
#[cfg(test)]
mod tests {
    use super::*;

    //  Page 26
    #[test]
    fn should_create_4x4() {
        let m = matrix4x4(
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5]
        );
        assert(float::equals(m[0][0], 1.0));
        assert(float::equals(m[0][3], 4.0));
        assert(float::equals(m[1][0], 5.5));
        assert(float::equals(m[1][2], 7.5));
        assert(float::equals(m[2][2], 11.0));
        assert(float::equals(m[3][0], 13.5));
        assert(float::equals(m[3][2], 15.5));
    }

    #[test]
    fn should_create_2x2() {
        let m = matrix2x2(
            [-3.0, 5.0],
            [1.0, -2.0]
        );
        assert(float::equals(m[0][0], -3.0));
        assert(float::equals(m[0][1], 5.0));
        assert(float::equals(m[1][0], 1.0));
        assert(float::equals(m[1][1], -2.0));
    }

    #[test]
    fn should_create_3x3() {
        let m = matrix3x3(
            [-3.0, 5.0, 0.0],
            [1.0, -2.0, -7.0],
            [0.0, 1.0, 1.0]
        );
        assert(float::equals(m[0][0], -3.0));
        assert(float::equals(m[0][1], 5.0));
        assert(float::equals(m[1][0], 1.0));
        assert(float::equals(m[1][1], -2.0));
    }
}