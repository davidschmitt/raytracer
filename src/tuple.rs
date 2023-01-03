use crate::float::F64IsAbout;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub trait TupleMethods {
    fn add(&self, peer: &Tuple) -> Tuple;
    fn subtract(&self, peer: &Tuple) -> Tuple;
    fn multiply(&self, scalar: f64) -> Tuple;
    fn divide(&self, scalar: f64) -> Tuple;
    fn negate(&self) -> Tuple;
    fn dot(&self, peer: &Tuple) -> f64;
    fn cross(&self, peer: &Tuple) -> Tuple;
    fn normalize(&self) -> Tuple;
    fn magnitude(&self) -> f64;
    fn is_point(&self) -> bool;
    fn is_vector(&self) -> bool;
    fn equals(&self, peer: &Tuple) -> bool;
}

impl TupleMethods for Tuple {
    fn add(self: &Tuple, peer: &Tuple) -> Tuple {
        Tuple {
            x: self.x + peer.x,
            y: self.y + peer.y,
            z: self.z + peer.z,
            w: self.w + peer.w,
        }
    }

    fn subtract(self: &Tuple, peer: &Tuple) -> Tuple {
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

    fn dot(self: &Tuple, peer: &Tuple) -> f64 {
        return self.x * peer.x + self.y * peer.y + self.z * peer.z + self.w * peer.w;
    }

    fn cross(self: &Tuple, peer: &Tuple) -> Tuple {
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
        self.w.is_about(1.0)
    }

    fn is_vector(self: &Tuple) -> bool {
        self.w.is_about(0.0)
    }
    fn equals(self: &Tuple, peer: &Tuple) -> bool {
        self.x.is_about(peer.x)
            && self.y.is_about(peer.y)
            && self.z.is_about(peer.z)
            && self.w.is_about(peer.w)
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
        assert!(point.x.is_about(x));
        assert!(point.y.is_about(y));
        assert!(point.z.is_about(z));
        assert!(point.w.is_about(w));
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
        assert!(vector.x.is_about(x));
        assert!(vector.y.is_about(y));
        assert!(vector.z.is_about(z));
        assert!(vector.w.is_about(w));
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
        assert!(tuple.x.is_about(point.x));
        assert!(tuple.y.is_about(point.y));
        assert!(tuple.z.is_about(point.z));
        assert!(tuple.w.is_about(point.w));
        assert!(tuple.w.is_about(w));
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
        assert!(tuple.x.is_about(vector.x));
        assert!(tuple.y.is_about(vector.y));
        assert!(tuple.z.is_about(vector.z));
        assert!(tuple.w.is_about(vector.w));
        assert!(tuple.w.is_about(w));
    }

    // Page 6
    #[test]
    fn should_add_tuples_correctly() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);

        let pv = p.add(&v);
        assert!(pv.x.is_about(1.0));
        assert!(pv.y.is_about(1.0));
        assert!(pv.z.is_about(6.0));
        assert!(pv.w.is_about(1.0));
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
        assert!(vector(1.0, 0.0, 0.0).magnitude().is_about(1.0));
        assert!(vector(0.0, 1.0, 0.0).magnitude().is_about(1.0));
        assert!(vector(0.0, 0.0, 1.0).magnitude().is_about(1.0));
        assert!(vector(1.0, 2.0, 3.0).magnitude().is_about(f64::sqrt(14.0)));
        assert!(vector(-1.0, -2.0, -3.0).magnitude().is_about(f64::sqrt(14.0)));
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
        assert!(a.dot(&b).is_about(20.0));
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
