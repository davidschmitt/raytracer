use crate::float;

pub type Tuple = (f64, f64, f64, f64);

pub fn negate(value:&Tuple) -> Tuple {
    return (-value.0, -value.1, -value.2, -value.3)
}

pub fn equals(a:&Tuple, b:&Tuple) -> bool {
    return float::equals(a.0, b.0) &&
    float::equals(a.1, b.1) &&
    float::equals(a.2, b.2) &&
    float::equals(a.3, b.3);
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 7
    #[test]
    fn should_negate_tuples() {
        let expected: Tuple = (-1.0, 2.0, -3.0, 4.0);
        let a: Tuple = (1.0, -2.0, 3.0, -4.0);
        let result = negate(&a);
        assert!(equals(&expected, &result));
    }
}