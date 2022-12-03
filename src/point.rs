use tuple;

pub struct PointOrVector {
    x:f64,
    y:f64,
    z:f64,
    w:f64,
}

pub fn tuple(value:&PointOrVector) -> Tuple {
    return (value.x, value.y, value.z, value.w);
}

pub fn from_tuple(value: Tuple) -> PointOrVector{
    return PointOrVector {
        x:value.0,
        y:value.1,
        z:value.2,
        w:value.3
    }
}

pub fn add(a:&PointOrVector, b:&PointOrVector) -> PointOrVector{
    assert!(f64_equals(a.w, 0.0) || f64_equals(b.w, 0.0));
    PointOrVector {
        x:a.x + b.x,
        y:a.y + b.y,
        z:a.z + b.z,
        w:a.w + b.w
    }
}

pub fn subtract(a:&PointOrVector, b:&PointOrVector) -> PointOrVector{
    assert!(!(f64_equals(a.w, 0.0) && f64_equals(b.w, 1.0)));
    PointOrVector {
        x:a.x - b.x,
        y:a.y - b.y,
        z:a.z - b.z,
        w:a.w - b.w
    }
}

pub fn point(x:f64, y:f64, z:f64) -> PointOrVector {
    PointOrVector {
        x:x,
        y:y,
        z:z,
        w:1.0
    }
}

pub fn vector(x:f64, y:f64, z:f64) -> PointOrVector {
    PointOrVector {
        x:x,
        y:y,
        z:z,
        w:0.0
    }
}

pub fn is_point(value:&PointOrVector) -> bool {
    return f64_equals(value.w, 1.0);
}

pub fn is_vector(value:&PointOrVector) -> bool {
    return f64_equals(value.w, 0.0);
}

const EPSILON:f64 = 0.00001;
pub fn f64_equals(a:f64, b:f64) -> bool {
    return (a - b).abs() < EPSILON;
}

pub fn equals(a:&PointOrVector, b:&PointOrVector) -> bool {
    return f64_equals(a.x, b.x) &&
    f64_equals(a.y, b.y) &&
    f64_equals(a.z, b.z) &&
    f64_equals(a.w, b.w);
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
        let point = from_tuple((x,y,z,w));
        assert!(f64_equals(point.x, x));
        assert!(f64_equals(point.y, y));
        assert!(f64_equals(point.z, z));
        assert!(f64_equals(point.w, w));
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
        let vector = from_tuple((x,y,z,w));
        assert!(f64_equals(vector.x, x));
        assert!(f64_equals(vector.y, y));
        assert!(f64_equals(vector.z, z));
        assert!(f64_equals(vector.w, w));
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
        let point = point(x , y, z);
        let tuple = tuple(&point);
        assert!(f64_equals(tuple.0, point.x));
        assert!(f64_equals(tuple.1, point.y));
        assert!(f64_equals(tuple.2, point.z));
        assert!(f64_equals(tuple.3, point.w));
        assert!(f64_equals(tuple.3, w));
    }

    // Page 4
    #[test]
    fn should_convert_vector_to_tuple() {
        let x = 4.3;
        let y = -4.2;
        let z = 3.1;
        let w: f64 = 0.0;
        let vector = vector(x , y, z);
        let tuple = tuple(&vector);
        assert!(f64_equals(tuple.0, vector.x));
        assert!(f64_equals(tuple.1, vector.y));
        assert!(f64_equals(tuple.2, vector.z));
        assert!(f64_equals(tuple.3, vector.w));
        assert!(f64_equals(tuple.3, w));
    }

    // Page 6
    #[test]
    fn should_add_tuples_correctly() {
        let p = point(3.0, -2.0, 5.0);
        let v = vector(-2.0, 3.0, 1.0);

        let pv = add(&p, &v);
        assert!(f64_equals(pv.x, 1.0));
        assert!(f64_equals(pv.y, 1.0));
        assert!(f64_equals(pv.z, 6.0));
        assert!(f64_equals(pv.w, 1.0));
    }

    // Page 6
    #[test]
    #[should_panic]
    fn should_not_add_two_points() {
        let p = point(3.0, -2.0, 5.0);
        add(&p, &p);
    }

    // Page 6
    #[test]
    fn should_compare_point_or_vectors() {
        let expected = from_tuple((0.0, 0.0, 0.0, 0.0));
        for x in 0..1 {
            for y in 0..1 {
                for z in 0..1 {
                    for w in 0..1 {
                        let value = from_tuple((f64::from(x), f64::from(y), f64::from(z), f64::from(w)));
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
        let p2 = point(5.0,6.0,7.0);
        let result = subtract(&p1, &p2);
        assert!(equals(&result, &expected));
    }

    // Page 6
    #[test]
    fn should_subtract_vector_from_point() {
        let expected = point(-2.0, -4.0, -6.0);
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0,6.0,7.0);
        let result = subtract(&p, &v);
        assert!(equals(&result, &expected));
    }

    // Page 7
    #[test]
    fn should_subtract_vector_from_vector() {
        let expected = vector(-2.0, -4.0, -6.0);
        let v1 = vector(3.0, 2.0, 1.0);
        let v2 = vector(5.0,6.0,7.0);
        let result = subtract(&v1, &v2);
        assert!(equals(&result, &expected));
    }

    #[test]
    #[should_panic]
    fn should_not_subtract_point_from_vector() {
        let p = point(3.0, 2.0, 1.0);
        let v = vector(5.0,6.0,7.0);
        subtract(&v, &p);
    }

    #[test]
    fn should negate tuples
}