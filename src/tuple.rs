use crate::float;

#[derive(Clone, Copy, Debug)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub fn tuple(x: f64, y: f64, z: f64, w: f64) -> Tuple {
    return Tuple { x, y, z, w };
}

pub fn add(a: &Tuple, b: &Tuple) -> Tuple {
    Tuple {
        x: a.x + b.x,
        y: a.y + b.y,
        z: a.z + b.z,
        w: a.w + b.w,
    }
}

pub fn subtract(a: &Tuple, b: &Tuple) -> Tuple {
    Tuple {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
}

pub fn multiply(tuple: &Tuple, scalar: f64) -> Tuple {
    Tuple {
        x: tuple.x * scalar,
        y: tuple.y * scalar,
        z: tuple.z * scalar,
        w: tuple.w * scalar,
    }
}

pub fn divide(tuple: &Tuple, scalar: f64) -> Tuple {
    Tuple {
        x: tuple.x / scalar,
        y: tuple.y / scalar,
        z: tuple.z / scalar,
        w: tuple.w / scalar,
    }
}

pub fn negate(tuple: &Tuple) -> Tuple {
    Tuple {
        x: -tuple.x,
        y: -tuple.y,
        z: -tuple.z,
        w: -tuple.w,
    }
}

pub fn dot(a: &Tuple, b: &Tuple) -> f64 {
    return a.x * b.x + a.y * b.y + a.z * b.z + a.w * b.w;
}

pub fn cross(a: &Tuple, b: &Tuple) -> Tuple {
    return Tuple {
        x: a.y * b.z - a.z * b.y,
        y: a.z * b.x - a.x * b.z,
        z: a.x * b.y - a.y * b.x,
        w: 0.0,
    };
}

pub fn magnitude(tuple: &Tuple) -> f64 {
    return f64::sqrt(
        tuple.x * tuple.x + tuple.y * tuple.y + tuple.z * tuple.z + tuple.w * tuple.w,
    );
}

pub fn normalize(tuple: &Tuple) -> Tuple {
    let magnitude = magnitude(tuple);
    return Tuple {
        x: tuple.x / magnitude,
        y: tuple.y / magnitude,
        z: tuple.z / magnitude,
        w: tuple.w / magnitude,
    };
}

pub fn point(x: f64, y: f64, z: f64) -> Tuple {
    return tuple(x, y, z, 1.0);
}

pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
    return tuple(x, y, z, 0.0);
}

pub fn is_point(value: &Tuple) -> bool {
    return float::equals(value.w, 1.0);
}

pub fn is_vector(value: &Tuple) -> bool {
    return float::equals(value.w, 0.0);
}

pub fn equals(a: &Tuple, b: &Tuple) -> bool {
    return float::equals(a.x, b.x)
        && float::equals(a.y, b.y)
        && float::equals(a.z, b.z)
        && float::equals(a.w, b.w);
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
        assert!(float::equals(point.x, x));
        assert!(float::equals(point.y, y));
        assert!(float::equals(point.z, z));
        assert!(float::equals(point.w, w));
        assert!(is_point(&point));
        assert!(!is_vector(&point));
    }

    // Page 4
    #[test]
    fn should_create_valid_vector() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w: f64 = 0.0;
        let vector = tuple(x, y, z, w);
        assert!(float::equals(vector.x, x));
        assert!(float::equals(vector.y, y));
        assert!(float::equals(vector.z, z));
        assert!(float::equals(vector.w, w));
        assert!(!is_point(&vector));
        assert!(is_vector(&vector));
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
        assert!(float::equals(tuple.x, point.x));
        assert!(float::equals(tuple.y, point.y));
        assert!(float::equals(tuple.z, point.z));
        assert!(float::equals(tuple.w, point.w));
        assert!(float::equals(tuple.w, w));
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
        assert!(float::equals(tuple.x, vector.x));
        assert!(float::equals(tuple.y, vector.y));
        assert!(float::equals(tuple.z, vector.z));
        assert!(float::equals(tuple.w, vector.w));
        assert!(float::equals(tuple.w, w));
    }

    // Page 6
    #[test]
    fn should_add_tuples_correctly() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);

        let pv = add(&p, &v);
        assert!(float::equals(pv.x, 1.0));
        assert!(float::equals(pv.y, 1.0));
        assert!(float::equals(pv.z, 6.0));
        assert!(float::equals(pv.w, 1.0));
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
                            assert!(!equals(&expected, &value));
                        } else {
                            assert!(equals(&expected, &value));
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
        let result = subtract(&p1, &p2);
        assert!(equals(&result, &expected));
    }

    // Page 6
    #[test]
    fn should_subtract_vector_from_point() {
        let expected = point(-2.0, -4.0, -6.0);
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0, 6.0, 7.0);
        let result = subtract(&p, &v);
        assert!(equals(&result, &expected));
    }

    // Page 7
    #[test]
    fn should_subtract_vector_from_vector() {
        let expected = vector(-2.0, -4.0, -6.0);
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0, 6.0, 7.0);
        let result = subtract(&v1, &v2);
        assert!(equals(&result, &expected));
    }

    // Page 7
    #[test]
    fn should_negate() {
        let expected: Tuple = tuple(-1.0, 2.0, -3.0, 4.0);
        let a: Tuple = tuple(1.0, -2.0, 3.0, -4.0);
        let result = negate(&a);
        assert!(equals(&expected, &result));
    }

    // Page 8
    #[test]
    fn should_do_scalar_multiply() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let result1 = multiply(&a, 3.5);
        let expected1 = tuple(3.5, -7.0, 10.5, -14.0);
        assert!(equals(&result1, &expected1));

        let result2 = multiply(&a, 0.5);
        let expected2 = tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equals(&result2, &expected2));
    }

    // Page 8
    #[test]
    fn should_do_scalar_divide() {
        let a = tuple(1.0, -2.0, 3.0, -4.0);
        let result = divide(&a, 2.0);
        let expected = tuple(0.5, -1.0, 1.5, -2.0);
        assert!(equals(&result, &expected));
    }

    // Page 8
    #[test]
    fn should_calculate_magnitude() {
        assert!(float::equals(magnitude(&vector(1.0, 0.0, 0.0)), 1.0));
        assert!(float::equals(magnitude(&vector(0.0, 1.0, 0.0)), 1.0));
        assert!(float::equals(magnitude(&vector(0.0, 0.0, 1.0)), 1.0));
        assert!(float::equals(
            magnitude(&vector(1.0, 2.0, 3.0)),
            f64::sqrt(14.0)
        ));
        assert!(float::equals(
            magnitude(&vector(-1.0, -2.0, -3.0)),
            f64::sqrt(14.0)
        ));
    }

    #[test]
    fn should_normalize_vectors() {
        assert!(equals(
            &normalize(&vector(4.0, 0.0, 0.0)),
            &vector(1.0, 0.0, 0.0)
        ));
    }

    // Page 10
    #[test]
    fn should_calculate_dot_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!(float::equals(dot(&a, &b), 20.0));
    }

    // Page 11
    #[test]
    fn should_calculate_cross_product() {
        let a = vector(1.0, 2.0, 3.0);
        let b = vector(2.0, 3.0, 4.0);
        assert!(equals(&cross(&a, &b), &vector(-1.0, 2.0, -1.0)));
        assert!(equals(&cross(&b, &a), &vector(1.0, -2.0, 1.0)));
    }
}
