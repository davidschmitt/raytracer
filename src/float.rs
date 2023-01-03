const EPSILON: f64 = 0.00001;

pub trait F64IsAbout {
    fn is_about(&self, peer: f64) -> bool;
}

impl F64IsAbout for f64 {
    fn is_about(self: &f64, peer: f64) -> bool {
        return (self - peer).abs() < EPSILON;
    }
}