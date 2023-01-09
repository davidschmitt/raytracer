use crate::float::F64IsAbout;
use std::cmp::{PartialEq};
use std::convert::AsRef;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl AsRef<Tuple> for Tuple {
    fn as_ref(&self) -> &Tuple {
        return self;
    }
}

impl PartialEq<Tuple> for Tuple {
    fn eq(&self, other: &Tuple) -> bool {
        return self.equals(other);
    }

    fn ne(&self, other: &Tuple) -> bool {
        return !self.equals(other);
    }
}    

pub trait TupleMethods {
    fn sum<T: AsRef<Tuple>>(&self, peer: T) -> Tuple;
    fn subtract<T: AsRef<Tuple>>(&self, peer: T) -> Tuple;
    fn multiply(&self, scalar: f64) -> Tuple;
    fn divide(&self, scalar: f64) -> Tuple;
    fn negate(&self) -> Tuple;
    fn dot<T: AsRef<Tuple>>(&self, peer: T) -> f64;
    fn cross<T: AsRef<Tuple>>(&self, peer: T) -> Tuple;
    fn normalize(&self) -> Tuple;
    fn magnitude(&self) -> f64;
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn equals<T: AsRef<Tuple>>(&self, peer: T) -> bool;
}

impl TupleMethods for Tuple {
    fn sum<T: AsRef<Tuple>>(self: &Tuple, other: T) -> Tuple {
        let peer = other.as_ref();
        Tuple {
            x: self.x + peer.x,
            y: self.y + peer.y,
            z: self.z + peer.z,
            w: self.w + peer.w,
        }
    }

    fn subtract<T: AsRef<Tuple>>(self: &Tuple, other: T) -> Tuple {
        let peer = other.as_ref();
        Tuple {
            x: self.x - peer.x,
            y: self.y - peer.y,
            z: self.z - peer.z,
            w: self.w - peer.w,
        }
    }

    fn multiply(self: &Tuple, scalar: f64) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w * scalar,
        }
    }

    fn divide(self: &Tuple, scalar: f64) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w / scalar,
        }
    }

    fn negate(self: &Tuple) -> Tuple {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }

    fn dot<T: AsRef<Tuple>>(self: &Tuple, other: T) -> f64 {
        let peer = other.as_ref();
        return self.x * peer.x + self.y * peer.y + self.z * peer.z + self.w * peer.w;
    }

    fn cross<T: AsRef<Tuple>>(self: &Tuple, other: T) -> Tuple {
        let peer = other.as_ref();
        return Tuple {
            x: self.y * peer.z - self.z * peer.y,
            y: self.z * peer.x - self.x * peer.z,
            z: self.x * peer.y - self.y * peer.x,
            w: 0.0,
        };
    }

    fn magnitude(self: &Tuple) -> f64 {
        return f64::sqrt(
            self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w,
        );
    }

    fn normalize(self: &Tuple) -> Tuple {
        let magnitude = self.magnitude();
        return Tuple {
            x: self.x / magnitude,
            y: self.y / magnitude,
            z: self.z / magnitude,
            w: self.w / magnitude,
        };
    }

    fn is_point(self: &Tuple) -> bool {
        self.w.equals(1.0)
    }

    fn is_vector(self: &Tuple) -> bool {
        self.w.equals(0.0)
    }

    fn equals<T: AsRef<Tuple>>(self: &Tuple, other: T) -> bool {
        let peer = other.as_ref();
        self.x.equals(peer.x)
            && self.y.equals(peer.y)
            && self.z.equals(peer.z)
            && self.w.equals(peer.w)
    }

}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    return Tuple { x, y, z, w };
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    return tuple(x, y, z, 1.0);
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    return tuple(x, y, z, 0.0);
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
        let w: f64 = 1.0;
        let point = tuple(x, y, z, w);
        assert!(point.x.equals(x));
        assert!(point.y.equals(y));
        assert!(point.z.equals(z));
        assert!(point.w.equals(w));
        assert!(point.is_point());
        assert!(!point.is_vector());
    }

    // Page 4
    #[test]
    fn should_create_valid_vector() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w: f64 = 0.0;
        let vector = tuple(x, y, z, w);
        assert!(vector.x.equals(x));
        assert!(vector.y.equals(y));
        assert!(vector.z.equals(z));
        assert!(vector.w.equals(w));
        assert!(!vector.is_point());
        assert!(vector.is_vector());
    }

    // Page 4
    #[test]
    fn should_convert_point_to_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w: f64 = 1.0;
        let point = point(x, y, z);
        let tuple = tuple(x, y, z, w);
        assert!(tuple.x.equals(point.x));
        assert!(tuple.y.equals(point.y));
        assert!(tuple.z.equals(point.z));
        assert!(tuple.w.equals(point.w));
        assert!(tuple.w.equals(w));
    }

    // Page 4
    #[test]
    fn should_convert_vector_to_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w: f64 = 0.0;
        let vector = vector(x, y, z);
        let tuple = tuple(x, y, z, w);
        assert!(tuple.x.equals(vector.x));
        assert!(tuple.y.equals(vector.y));
        assert!(tuple.z.equals(vector.z));
        assert!(tuple.w.equals(vector.w));
        assert!(tuple.w.equals(w));
    }

    // Page 6
    #[test]
    fn should_add_tuples_correctly() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);

        let pv = p.sum(&v);
        assert!(pv.x.equals(1.0));
        assert!(pv.y.equals(1.0));
        assert!(pv.z.equals(6.0));
        assert!(pv.w.equals(1.0));
    }

    // Page 6
    #[test]
    fn should_compare_point_or_vectors() {
        let expected = tuple(0.0, 0.0, 0.0, 0.0);
        for x in 0..2 {
            for y in 0..2 {
                for z in 0..2 {
                    for w in 0..2 {
                        let value = tuple(f64::from(x), f64::from(y), f64::from(z), f64::from(w));
                        if x + y + z + w > 0 {
                            assert!(!expected.equals(&value));
                        } else {
                            assert!(expected.equals(&value));
                        }
                    }
                }
            }
        }
    }

    // Page 6
    #[test]
    fn should_subtract_point_from_point() {
        let expected = vector(-2.0, -4.0, -6.0);
        let p1 = point(3.0, 2.0, 1.0);
        let p2 = point(5.0, 6.0, 7.0);
        let result = p1.subtract(&p2);
        assert!(result.equals(&expected));
    }

    // Page 6
    #[test]
    fn should_subtract_vector_from_point() {
        let expected = point(-2.0, -4.0, -6.0);
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let result = p.subtract(&v);
        assert!(&result.equals(&expected));
    }

    // Page 7
    #[test]
    fn should_subtract_vector_from_vector() {
        let expected = vector(-2.0, -4.0, -6.0);
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let result = v1.subtract(&v2);
        assert!(&result.equals(&expected));
    }

    // Page 7
    #[test]
    fn should_negate() {
        let expected: Tuple = tuple(-1.0, 2.0, -3.0, 4.0);
        let a: Tuple = tuple(1.0, -2.0, 3.0, -4.0);
        let result = a.negate();
        assert!(expected.equals(&result));
    }

    // Page 8
    #[test]
    fn should_do_scalar_multiply() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let result1 = a.multiply(3.5);
        let expected1 = tuple(3.5, -7.0, 10.5, -14.0);
        assert!(&result1.equals(&expected1));

        let result2 = a.multiply(0.5);
        let expected2 = tuple(0.5, -1.0, 1.5, -2.0);
        assert!(&result2.equals(&expected2));
    }

    // Page 8
    #[test]
    fn should_do_scalar_divide() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let result = a.divide(2.0);
        let expected = tuple(0.5, -1.0, 1.5, -2.0);
        assert!(&result.equals(&expected));
    }

    // Page 8
    #[test]
    fn should_calculate_magnitude() {
        assert!(vector(1.0, 0.0, 0.0).magnitude().equals(1.0));
        assert!(vector(0.0, 1.0, 0.0).magnitude().equals(1.0));
        assert!(vector(0.0, 0.0, 1.0).magnitude().equals(1.0));
        assert!(vector(1.0, 2.0, 3.0).magnitude().equals(f64::sqrt(14.0)));
        assert!(vector(-1.0, -2.0, -3.0).magnitude().equals(f64::sqrt(14.0)));
    }

    #[test]
    fn should_normalize_vectors() {
        assert!(
            vector(4.0, 0.0, 0.0).normalize().equals(&vector(1.0, 0.0, 0.0))
        );
    }

    // Page 10
    #[test]
    fn should_calculate_dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!(a.dot(&b).equals(20.0));
    }

    // Page 11
    #[test]
    fn should_calculate_cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!(a.cross(&b).equals(&vector(-1.0, 2.0, -1.0)));
        assert!(b.cross(&a).equals(&vector(1.0, -2.0, 1.0)));
    }
}
