use crate::matrix4;
use crate::ray;
use crate::tuple;
use crate::tuple::TupleMethods;

pub fn translation(x: f64, y: f64, z: f64) -> matrix4::Matrix4 {
    return [
        [1.0, 0.0, 0.0, x],
        [0.0, 1.0, 0.0, y],
        [0.0, 0.0, 1.0, z],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn scaling(x: f64, y: f64, z: f64) -> matrix4::Matrix4 {
    return [
        [x, 0.0, 0.0, 0.0],
        [0.0, y, 0.0, 0.0],
        [0.0, 0.0, z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn rotation_x(rad: f64) -> matrix4::Matrix4 {
    return [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, rad.cos(), -rad.sin(), 0.0],
        [0.0, rad.sin(), rad.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn rotation_y(rad: f64) -> matrix4::Matrix4 {
    return [
        [rad.cos(), 0.0, rad.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [-rad.sin(), 0.0, rad.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn rotation_z(rad: f64) -> matrix4::Matrix4 {
    return [
        [rad.cos(), -rad.sin(), 0.0, 0.0],
        [rad.sin(), rad.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}

pub fn shearing(xmy: f64, xmz: f64, ymx: f64, ymz: f64, zmx: f64, zmy: f64) -> matrix4::Matrix4 {
    return [
        [1.0, xmy, xmz, 0.0],
        [ymx, 1.0, ymz, 0.0],
        [zmx, zmy, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ];
}


#[cfg(test)]
mod tests {
    use crate::matrix4::Matrix4Methods;

    use super::*;

    //  Page 45
    #[test]
    fn should_translate() {
        let transform = translation(5.0, -3.0, 2.0);
        let p = tuple::point(-3.0, 4.0, 5.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(2.0, 1.0, 7.0);
        assert!(result.equals(&expected));
    }

    //  Page 45
    #[test]
    fn should_translate_in_reverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = tuple::point(-3.0, 4.0, 5.0);
        let result = inv.times_tuple(&p);
        let expected = tuple::point(-8.0, 7.0, 3.0);
        assert!(result.equals(&expected));
    }

    //  Page 45
    #[test]
    fn should_not_translate_vectors() {
        let transform = translation(5.0, -3.0, 2.0);
        let v = tuple::vector(-3.0, 4.0, 5.0);
        let result = transform.times_tuple(&v);
        assert!(result.equals(&v));
    }

    //  Page 46
    #[test]
    fn should_scale_a_point() {
        let transform = scaling(2.0, 3.0, 4.0);
        let p = tuple::point(-4.0, 6.0, 8.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(-8.0, 18.0, 32.0);
        assert!(result.equals(&expected));
    }

    //  Page 46
    #[test]
    fn should_scale_a_vector() {
        let transform = scaling(2.0, 3.0, 4.0);
        let v = tuple::vector(-4.0, 6.0, 8.0);
        let result = transform.times_tuple(&v);
        let expected = tuple::vector(-8.0, 18.0, 32.0);
        assert!(result.equals(&expected));
    }

    //  Page 46
    #[test]
    fn should_scale_inversely() {
        let transform = scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = tuple::vector(-4.0, 6.0, 8.0);
        let result = inv.times_tuple(&v);
        let expected = tuple::vector(-2.0, 2.0, 2.0);
        assert!(result.equals(&expected));
    }

    //  Page 47
    #[test]
    fn should_reflect() {
        let transform = scaling(-1.0, 1.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(-2.0, 3.0, 4.0);
        assert!(result.equals(&expected));
    }

    //  Page 48
    #[test]
    fn should_rotate_around_x_axis() {
        let p = tuple::point(0.0, 1.0, 0.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let r1 = half_quarter.times_tuple(&p);
        let e1 = tuple::point(0.0, two.sqrt() / 2.0, two.sqrt() / 2.0);
        assert!(r1.equals(&e1));

        let full_quarter = rotation_x(std::f64::consts::PI / 2.0);
        let r2 = full_quarter.times_tuple(&p);
        let e2 = tuple::point(0.0, 0.0, 1.0);
        assert!(r2.equals(&e2));
    }

    //  Page 49
    #[test]
    fn should_inverse_rotate_around_x_axis() {
        let two: f64 = 2.0;
        let p = tuple::point(0.0, 1.0, 0.0);
        let half_quarter = rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        let result = inv.times_tuple(&p);
        let expected = tuple::point(0.0, two.sqrt() / 2.0, -two.sqrt() / 2.0);
        assert!(result.equals(&expected));
    }

    //  Page 49
    #[test]
    fn should_rotate_around_y_axis() {
        let p = tuple::point(0.0, 0.0, 1.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_y(std::f64::consts::PI / 4.0);
        let r1 = half_quarter.times_tuple(&p);
        let e1 = tuple::point(two.sqrt() / 2.0, 0.0, two.sqrt() / 2.0);
        assert!(r1.equals(&e1));

        let full_quarter = rotation_y(std::f64::consts::PI / 2.0);
        let r2 = full_quarter.times_tuple(&p);
        let e2 = tuple::point(1.0, 0.0, 0.0);
        assert!(r2.equals(&e2));
    }

    //  Page 50
    #[test]
    fn should_rotate_around_z_axis() {
        let p = tuple::point(0.0, 1.0, 0.0);
        let two: f64 = 2.0;

        let half_quarter = rotation_z(std::f64::consts::PI / 4.0);
        let r1 = half_quarter.times_tuple(&p);
        let e1 = tuple::point(-two.sqrt() / 2.0, two.sqrt() / 2.0, 0.0);
        assert!(r1.equals(&e1));

        let full_quarter = rotation_z(std::f64::consts::PI / 2.0);
        let r2 = full_quarter.times_tuple(&p);
        let e2 = tuple::point(-1.0, 0.0, 0.0);
        assert!(r2.equals(&e2));
    }

    //  Page 52
    #[test]
    fn should_shear_x_to_y() {
        let transform = shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(5.0, 3.0, 4.0);
        assert!(result.equals(&expected));
    }
    //  Page 52
    #[test]
    fn should_shear_x_to_z() {
        let transform = shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(6.0, 3.0, 4.0);
        assert!(result.equals(&expected));
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_x() {
        let transform = shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(2.0, 5.0, 4.0);
        assert!(result.equals(&expected));
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_z() {
        let transform = shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(2.0, 7.0, 4.0);
        assert!(result.equals(&expected));
    }
    //  Page 52
    #[test]
    fn should_shear_z_to_x() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(2.0, 3.0, 6.0);
        assert!(result.equals(&expected));
    }
    //  Page 53
    #[test]
    fn should_shear_z_to_y() {
        let transform = shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = tuple::point(2.0, 3.0, 4.0);
        let result = transform.times_tuple(&p);
        let expected = tuple::point(2.0, 3.0, 7.0);
        assert!(result.equals(&expected));
    }

    //  Page 54
    #[test]
    fn should_transform_in_sequence() {
        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let p2 = a.times_tuple(&p);
        let e2 = tuple::point(1.0, -1.0, 0.0);
        assert!(p2.equals(&e2));
        let p3 = b.times_tuple(&p2);
        let e3 = tuple::point(5.0, -5.0, 0.0);
        assert!(p3.equals(&e3));
        let p4 = c.times_tuple(&p3);
        let e4 = tuple::point(15.0, 0.0, 7.0);
        assert!(p4.equals(&e4));
    }

    //  Page 54
    #[test]
    fn should_transform_chained() {
        let p = tuple::point(1.0, 0.0, 1.0);
        let a = rotation_x(std::f64::consts::PI / 2.0);
        let b = scaling(5.0, 5.0, 5.0);
        let c = translation(10.0, 5.0, 7.0);
        let t = c.multiply(&b).multiply(&a);
        let result = t.times_tuple(&p);
        let expected = tuple::point(15.0, 0.0, 7.0);
        assert!(result.equals(&expected));
    }
}
