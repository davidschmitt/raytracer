use crate::matrix4;
use crate::tuple;

#[cfg(test)]
mod tests {
    use super::*;

    /*
    //  Page 45
    #[test]
    fn should_translate() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = tuple::point(-3.0, 4.0, 5.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(2.0, 1.0, 7.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 45
    #[test]
    fn should_translate_in_reverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = matrix4::inverse(transform);
        let p = tuple::point(-3.0, 4.0, 5.0);
        let result = matrix4::multiply_tuple(&inv, &p);
        let expected = tuple::point(-8.0, 7.0, 3.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 45
    #[test]
    fn should_not_translate_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = tuple::vector(-3.0, 4.0, 5.0);
        let result = matrix4::multiply_tuple(&transform, &v);
        assert!(tuple::equals(&result, &v));
    }

    //  Page 46
    #[test]
    fn should_scale_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = tuple::point(-4.0, 6.0, 8.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(-8.0, 18.0, 32.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 46
    #[test]
    fn should_scale_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = tuple::vector(-4.0, 6.0, 8.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::vector(-8.0, 18.0, 32.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 46
    #[test]
    fn should_scale_inversely() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = matrix4::inverse(&transform);
        let v = tuple::vector(-4.0, 6.0, 8.0);
        let result = matrix4::multiply_tuple(&inv, &p);
        let expected = tuple::vector(-2.0, 2.0, 2.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 47
    #[test]
    fn should_reflect() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::vector(-2.0, 3.0, 4.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 48
    #[test]
    fn should_rotate_around_x_axis() {
        let p = tuple::point(0.0, 1.0, 0.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let r1 = matrix4::multiply_tuple(&half_quarter, &p);
        let e1 = tuple::point(0.0, two.sqrt() / 2.0, two.sqrt() / 2.0);
        assert!(tuple::equals(&r1, &e1));

        let full_quarter = rotation_x(std::f64::consts::PI / 2.0);
        let r2 = matrix4::multiply_tuple(&full_quarter, &p);
        let e2 = tuple::point(0.0, 0.0, 1.0);
        assert!(tuple::equals(&r2, &e2));
    }

    //  Page 49
    #[test]
    fn should_inverse_rotate_around_x_axis() {
        let two: f64 = 2.0;
        let p = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let inv = matrix4::inverse(&half_quarter);
        let result = matrix4::multiply_tuple(&inv, &p);
        let expected = tuple::point(0.0, two.sqrt() / 2.0, -two.sqrt() / 2.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 49
    #[test]
    fn should_rotate_around_y_axis() {
        let p = tuple::point(0.0, 0.0, 1.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
        let r1 = matrix4::multiply_tuple(&half_quarter, &p);
        let e1 = tuple::point(two.sqrt() / 2.0, 0.0, two.sqrt() / 2.0);
        assert!(tuple::equals(&r1, &e1));

        let full_quarter = rotation_y(std::f64::consts::PI / 2.0);
        let r2 = matrix4::multiply_tuple(&full_quarter, &p);
        let e2 = tuple::point(1.0, 0.0, 0.0);
        assert!(tuple::equals(&r2, &e2));
    }

    //  Page 50
    #[test]
    fn should_rotate_around_z_axis() {
        let p = tuple::point(0.0, 1.0, 0.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
        let r1 = matrix4::multiply_tuple(&half_quarter, &p);
        let e1 = tuple::point(-two.sqrt() / 2.0, two.sqrt() / 2.0, 0.0);
        assert!(tuple::equals(&r1, &e1));

        let full_quarter = rotation_z(std::f64::consts::PI / 2.0);
        let r2 = matrix4::multiply_tuple(&full_quarter, &p);
        let e2 = tuple::point(-1.0, 0.0, 0.0);
        assert!(tuple::equals(&r2, &e2));
    }

    //  Page 52
    #[test]
    fn should_shear_x_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(5.0, 3.0, 4.0);
        assert!(tuple::equals(&result, &expected));
    }
    //  Page 52
    #[test]
    fn should_shear_x_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(6.0, 3.0, 4.0);
        assert!(tuple::equals(&result, &expected));
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(2.0, 5.0, 4.0);
        assert!(tuple::equals(&result, &expected));
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(2.0, 7.0, 4.0);
        assert!(tuple::equals(&result, &expected));
    }
    //  Page 52
    #[test]
    fn should_shear_z_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(2.0, 3.0, 6.0);
        assert!(tuple::equals(&result, &expected));
    }
    //  Page 53
    #[test]
    fn should_shear_z_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = matrix4::multiply_tuple(&transform, &p);
        let expected = tuple::point(2.0, 3.0, 7.0);
        assert!(tuple::equals(&result, &expected));
    }

    //  Page 54
    #[test]
    fn should_transform_in_sequence() {
        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(1.0, 5.0, 7.0);
        let p2 = matrix4::multiply_tuple(&a, &p);
        let e2 = tuple::point(1.0, -1.0, 0.0);
        assert!(tuple::equals(&p2, &e2));
        let p3 = matrix4::multiply_tuple(&b, &p2);
        let e3 = tuple::point(5.0, -5.0, 0.0);
        assert!(tuple::equals(&p3, &e3));
        let p4 = matrix4::multiply_tuple(&c, &p3);
        let e4 = tuple::point(15.0, 0.0, 7.0);
        assert!(tuple::equals(&p4, &e4));
    }

    //  Page 54
    #[test]
    fn should_transform_chained() {
        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(1.0, 5.0, 7.0);
        let t = matrix4::multiply(&matrix4::multiply(&c, &b), &a);
        let result = matrix4::multiply_tuple(&t, &p);
        let expected = tuple::point(15.0, 0.0, 7.0);
        assert!(tuple::equals(&result, &expected));
    }
    */
}
