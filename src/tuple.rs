use crate::float::Float;
use std::cmp::PartialEq;
use std::convert::{Into, AsRef};
use std::ops::{Add, Sub, Mul, Div, Neg};

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: Float,
    pub y: Float,
    pub z: Float,
    pub w: Float,
}

impl AsRef<Tuple> for Tuple {
    fn as_ref(&self) -> &Tuple {
        return self;
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, peer: &Tuple) -> bool {
        self.x == peer.x
            && self.y == peer.y
            && self.z == peer.z
            && self.w == peer.w
    }

    fn ne(&self, other: &Tuple) -> bool {
        return !self.eq(other);
    }
}

impl <S: AsRef<Tuple>> Add<S> for Tuple {
    type Output = Tuple;
    fn add(self, rhs: S) -> Self::Output {
        let peer = rhs.as_ref();
        return Tuple {
            x: self.x + peer.x,
            y: self.y + peer.y,
            z: self.z + peer.z,
            w: self.w + peer.w,
        }
    }
}

impl <S: AsRef<Tuple>> Sub<S> for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: S) -> Self::Output {
        let peer = rhs.as_ref();
        return Tuple {
            x: self.x - peer.x,
            y: self.y - peer.y,
            z: self.z - peer.z,
            w: self.w - peer.w,
        }
    }
}

impl <S: Into<f64>> Mul<S> for Tuple {
    type Output = Tuple;
    fn mul(self, scalar: S) -> Tuple {
        let s = scalar.into();
        return Tuple {
            x: self.x * s,
            y: self.y * s,
            z: self.z * s,
            w: self.w * s,
        }
    }
}

impl <S: Into<f64>> Div<S> for Tuple {
    type Output = Tuple;
    fn div(self, scalar: S) -> Tuple {
        // I assume one divide and four multiplies faster than four divides
        let inverse = 1.0 / scalar.into();
        return Tuple {
            x: self.x * inverse,
            y: self.y * inverse,
            z: self.z * inverse,
            w: self.w * inverse,
        }
    }
}

impl Neg for Tuple {
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        return Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}


impl Tuple {
    pub fn new<X: Into<f64>, Y: Into<f64>, Z: Into<f64>, W: Into<f64>>(x: X, y: Y, z: Z, w: W) -> Tuple {
        return Tuple { 
            x: Float::from(x.into()), 
            y: Float::from(y.into()),
            z: Float::from(z.into()),
            w: Float::from(w.into())
        };
    }

    pub fn point<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Tuple {
        return Tuple::new(x, y, z, 1);
    }

    pub fn vector<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Tuple {
        return Tuple::new(x, y, z, 0);
    }

    pub fn dot<T: AsRef<Tuple>>(self: &Tuple, other: T) -> Float {
        let peer = other.as_ref();
        return self.x * peer.x + self.y * peer.y + self.z * peer.z + self.w * peer.w;
    }

    pub fn cross<T: AsRef<Tuple>>(self: &Tuple, other: T) -> Tuple {
        let peer = other.as_ref();
        return Tuple {
            x: self.y * peer.z - self.z * peer.y,
            y: self.z * peer.x - self.x * peer.z,
            z: self.x * peer.y - self.y * peer.x,
            w: Float::from(0),
        };
    }

    pub fn magnitude(self: &Tuple) -> Float {
        let squared = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        return squared.sqrt();
    }

    pub fn normalize(self: &Tuple) -> Tuple {
        let magnitude = self.magnitude();
        return Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        };
    }

    pub fn is_point(self: &Tuple) -> bool {
        return self.w == 1;
    }

    pub fn is_vector(self: &Tuple) -> bool {
        return self.w == 0;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 4
    #[test]
    fn should_create_valid_point() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 1.0;
        let point = Tuple::new(x, y, z, w);
        assert!(point.x == x);
        assert!(point.y == y);
        assert!(point.z == z);
        assert!(point.w == w);
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    // Page 4
    #[test]
    fn should_create_valid_vector() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 0;
        let vector = Tuple::new(x, y, z, w);
        assert!(vector.x == x);
        assert!(vector.y == y);
        assert!(vector.z == z);
        assert!(vector.w == w);
        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    // Page 4
    #[test]
    fn should_convert_point_to_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 1;
        let point = Tuple::point(x, y, z);
        let tuple = Tuple::new(x, y, z, w);
        assert!(tuple.x == point.x);
        assert!(tuple.y == point.y);
        assert!(tuple.z == point.z);
        assert!(tuple.w == point.w);
        assert!(tuple.w == w);
    }

    // Page 4
    #[test]
    fn should_convert_vector_to_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w = 0;
        let vector = Tuple::vector(x, y, z);
        let tuple = Tuple::new(x, y, z, w);
        assert!(tuple.x == vector.x);
        assert!(tuple.y == vector.y);
        assert!(tuple.z == vector.z);
        assert!(tuple.w == vector.w);
        assert!(tuple.w == w);
    }

    // Page 6
    #[test]
    fn should_add_tuples_correctly() {
        let p = Tuple::point(3.0, -2.0, 5.0);
        let v = Tuple::vector(-2.0, 3.0, 1.0);

        let pv = p + v;
        assert!(pv.x == 1);
        assert!(pv.y == 1);
        assert!(pv.z == 6);
        assert!(pv.w == 1);
    }

    // Page 6
    #[test]
    fn should_compare_point_or_vectors() {
        let expected = Tuple::new(0, 0, 0, 0);
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    for w in 0..2 {
                        let value = Tuple::new(x, y, z, w);
                        if x + y + z + w > 0 {
                            assert!(expected != value);
                        } else {
                            assert!(expected == value);
                        }
                    }
                }
            }
        }
    }

    // Page 6
    #[test]
    fn should_subtract_point_from_point() {
        let expected = Tuple::vector(-2, -4, -6);
        let p1 = Tuple::point(3, 2, 1);
        let p2 = Tuple::point(5, 6, 7);
        let result = p1 - p2;
        assert!(result == expected);
    }

    // Page 6
    #[test]
    fn should_subtract_vector_from_point() {
        let expected = Tuple::point(-2, -4, -6);
        let p = Tuple::point(3, 2, 1);
        let v = Tuple::vector(5, 6, 7);
        let result = p - v;
        assert!(result == expected);
    }

    // Page 7
    #[test]
    fn should_subtract_vector_from_vector() {
        let expected = Tuple::vector(-2, -4, -6);
        let v1 = Tuple::vector(3, 2, 1);
        let v2 = Tuple::vector(5, 6, 7);
        let result = v1 - v2;
        assert!(result == expected);
    }

    // Page 7
    #[test]
    fn should_negate() {
        let expected = Tuple::new(-1, 2, -3, 4);
        let a = Tuple::new(1, -2, 3, -4);
        let result = -a;
        assert!(expected == result);
    }

    // Page 8
    #[test]
    fn should_do_scalar_multiply() {
        let a = Tuple::new(1, -2, 3, -4);
        let result1 = a * 3.5;
        let expected1 = Tuple::new(3.5, -7, 10.5, -14);
        assert!(result1 == expected1);

        let result2 = a * 0.5;
        let expected2 = Tuple::new(0.5, -1, 1.5, -2);
        assert!(result2 == expected2);
    }

    // Page 8
    #[test]
    fn should_do_scalar_divide() {
        let a = Tuple::new(1, -2, 3, -4);
        let result = a / 2;
        let expected = Tuple::new(0.5, -1, 1.5, -2);
        assert!(result == expected);
    }

    // Page 8
    #[test]
    fn should_calculate_magnitude() {
        let mag = 14.0_f64.sqrt();
        assert!(Tuple::vector(1, 0, 0).magnitude() == 1.0);
        assert!(Tuple::vector(0, 1, 0).magnitude() == 1.0);
        assert!(Tuple::vector(0, 0, 1).magnitude() == 1.0);
        assert!(Tuple::vector(1, 2, 3).magnitude() == mag);
        assert!(Tuple::vector(-1, -2, -3).magnitude() == mag);
    }

    #[test]
    fn should_normalize_vectors() {
        assert!(
            Tuple::vector(4, 0, 0).normalize() == Tuple::vector(1, 0, 0)
        );
    }

    // Page 10
    #[test]
    fn should_calculate_dot_product() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);
        assert!(a.dot(b) == 20.0);
    }

    // Page 11
    #[test]
    fn should_calculate_cross_product() {
        let a = Tuple::vector(1, 2, 3);
        let b = Tuple::vector(2, 3, 4);
        assert!(a.cross(b) == Tuple::vector(-1, 2, -1));
        assert!(b.cross(a) == Tuple::vector(1, -2, 1));
    }
}
