const EPSILON: f64 = 0.00001;
pub fn equals(a: f64, b: f64) -> bool {
    return (a - b).abs() < EPSILON;
}
