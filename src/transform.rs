use crate::matrix4::Matrix4;

pub struct Transform {}

impl Transform {
    pub fn translation<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Matrix4 {
        return Matrix4::from([
            [1.0, 0.0, 0.0, x.into()],
            [0.0, 1.0, 0.0, y.into()],
            [0.0, 0.0, 1.0, z.into()],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn scaling<X: Into<f64>, Y: Into<f64>, Z: Into<f64>>(x: X, y: Y, z: Z) -> Matrix4 {
        return Matrix4::from([
            [x.into(), 0.0, 0.0, 0.0],
            [0.0, y.into(), 0.0, 0.0],
            [0.0, 0.0, z.into(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_x<R: Into<f64>>(rad: R) -> Matrix4 {
        let r = rad.into();
        return Matrix4::from([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_y<R: Into<f64>>(rad: R) -> Matrix4 {
        let r = rad.into();
        return Matrix4::from([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn rotation_z<R: Into<f64>>(rad: R) -> Matrix4 {
        let r = rad.into();
        return Matrix4::from([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    pub fn shearing<
      XY: Into<f64>,
      XZ: Into<f64>,
      YX: Into<f64>,
      YZ: Into<f64>,
      ZX: Into<f64>,
      ZY: Into<f64>,
    >(xmy: XY, xmz: XZ, ymx: YX, ymz: YZ, zmx: ZX, zmy: ZY) -> Matrix4 {
        return Matrix4::from([
            [1.0, xmy.into(), xmz.into(), 0.0],
            [ymx.into(), 1.0, ymz.into(), 0.0],
            [zmx.into(), zmy.into(), 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);
    }

    
    pub fn identity() -> Matrix4 {
        return Matrix4::from([
            [1, 0, 0, 0],
            [0, 1, 0, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 1],
        ]);
    }
}


#[cfg(test)]
mod tests {
    use crate::tuple::Tuple;
    use super::*;

    //  Page 45
    #[test]
    fn should_translate() {
        let transform = Transform::translation(5, -3, 2);
        let p = Tuple::point(-3, 4, 5);
        let result = transform * p;
        let expected = Tuple::point(2, 1, 7);
        assert!(result == expected);
    }

    //  Page 45
    #[test]
    fn should_translate_in_reverse() {
        let transform = Transform::translation(5, -3, 2);
        let inv = transform.inverse();
        let p = Tuple::point(-3, 4, 5);
        let result = inv * p;
        let expected = Tuple::point(-8, 7, 3);
        assert!(result == expected);
    }

    //  Page 45
    #[test]
    fn should_not_translate_vectors() {
        let transform = Transform::translation(5, -3, 2);
        let v = Tuple::vector(-3, 4, 5);
        let result = transform *v;
        assert!(result == v);
    }

    //  Page 46
    #[test]
    fn should_scale_a_point() {
        let transform = Transform::scaling(2, 3, 4);
        let p = Tuple::point(-4, 6, 8);
        let result = transform * p;
        let expected = Tuple::point(-8, 18, 32);
        assert!(result == expected);
    }

    //  Page 46
    #[test]
    fn should_scale_a_vector() {
        let transform = Transform::scaling(2, 3, 4);
        let v = Tuple::vector(-4, 6, 8);
        let result = transform * v;
        let expected = Tuple::vector(-8, 18, 32);
        assert!(result == expected);
    }

    //  Page 46
    #[test]
    fn should_scale_inversely() {
        let transform = Transform::scaling(2, 3, 4);
        let inv = transform.inverse();
        let v = Tuple::vector(-4, 6, 8);
        let result = inv * v;
        let expected = Tuple::vector(-2, 2, 2);
        assert!(result == expected);
    }

    //  Page 47
    #[test]
    fn should_reflect() {
        let transform = Transform::scaling(-1, 1, 1);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(-2, 3, 4);
        assert!(result == expected);
    }

    //  Page 48
    #[test]
    fn should_rotate_around_x_axis() {
        let p = Tuple::point(0, 1, 0);
        let two: f64 = 2.0;

        let half_quarter = Transform::rotation_x(std::f64::consts::PI / 4.0);
        let r1 = half_quarter * p;
        let e1 = Tuple::point(0, two.sqrt() / 2.0, two.sqrt() / 2.0);
        assert!(r1 == e1);

        let full_quarter = Transform::rotation_x(std::f64::consts::PI / 2.0);
        let r2 = full_quarter * p;
        let e2 = Tuple::point(0, 0, 1);
        assert!(r2 == e2);
    }

    //  Page 49
    #[test]
    fn should_inverse_rotate_around_x_axis() {
        let two: f64 = 2.0;
        let p = Tuple::point(0, 1, 0);
        let half_quarter = Transform::rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        let result = inv * p;
        let expected = Tuple::point(0, two.sqrt() / 2.0, -two.sqrt() / 2.0);
        assert!(result == expected);
    }

    //  Page 49
    #[test]
    fn should_rotate_around_y_axis() {
        let p = Tuple::point(0, 0, 1);
        let two: f64 = 2.0;

        let half_quarter = Transform::rotation_y(std::f64::consts::PI / 4.0);
        let r1 = half_quarter * p;
        let e1 = Tuple::point(two.sqrt() / 2.0, 0, two.sqrt() / 2.0);
        assert!(r1 == e1);

        let full_quarter = Transform::rotation_y(std::f64::consts::PI / 2.0);
        let r2 = full_quarter * p;
        let e2 = Tuple::point(1, 0, 0);
        assert!(r2 == e2);
    }

    //  Page 50
    #[test]
    fn should_rotate_around_z_axis() {
        let p = Tuple::point(0, 1, 0);
        let two: f64 = 2.0;

        let half_quarter = Transform::rotation_z(std::f64::consts::PI / 4.0);
        let r1 = half_quarter * p;
        let e1 = Tuple::point(-two.sqrt() / 2.0, two.sqrt() / 2.0, 0);
        assert!(r1 == e1);

        let full_quarter = Transform::rotation_z(std::f64::consts::PI / 2.0);
        let r2 = full_quarter * p;
        let e2 = Tuple::point(-1, 0, 0);
        assert!(r2 == e2);
    }

    //  Page 52
    #[test]
    fn should_shear_x_to_y() {
        let transform = Transform::shearing(1, 0, 0, 0, 0, 0);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(5, 3, 4);
        assert!(result == expected);
    }
    //  Page 52
    #[test]
    fn should_shear_x_to_z() {
        let transform = Transform::shearing(0, 1, 0, 0, 0, 0);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(6, 3, 4);
        assert!(result == expected);
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_x() {
        let transform = Transform::shearing(0, 0, 1, 0, 0, 0);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(2, 5, 4);
        assert!(result == expected);
    }
    //  Page 52
    #[test]
    fn should_shear_y_to_z() {
        let transform = Transform::shearing(0, 0, 0, 1, 0, 0);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(2, 7, 4);
        assert!(result == expected);
    }
    //  Page 52
    #[test]
    fn should_shear_z_to_x() {
        let transform = Transform::shearing(0, 0, 0, 0, 1, 0);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(2, 3, 6);
        assert!(result == expected);
    }
    //  Page 53
    #[test]
    fn should_shear_z_to_y() {
        let transform = Transform::shearing(0, 0, 0, 0, 0, 1);
        let p = Tuple::point(2, 3, 4);
        let result = transform * p;
        let expected = Tuple::point(2, 3, 7);
        assert!(result == expected);
    }

    //  Page 54
    #[test]
    fn should_transform_in_sequence() {
        let p = Tuple::point(1, 0, 1);
        let a = Transform::rotation_x(std::f64::consts::PI / 2.0);
        let b = Transform::scaling(5, 5, 5);
        let c = Transform::translation(10, 5, 7);
        let p2 = a * p;
        let e2 = Tuple::point(1, -1, 0);
        assert!(p2 == e2);
        let p3 = b * p2;
        let e3 = Tuple::point(5, -5, 0);
        assert!(p3 == e3);
        let p4 = c * p3;
        let e4 = Tuple::point(15, 0, 7);
        assert!(p4 == e4);
    }

    //  Page 54
    #[test]
    fn should_transform_chained() {
        let p = Tuple::point(1, 0, 1);
        let a = Transform::rotation_x(std::f64::consts::PI / 2.0);
        let b = Transform::scaling(5, 5, 5);
        let c = Transform::translation(10, 5, 7);
        let t = c * b * a;
        let result = t * p;
        let expected = Tuple::point(15, 0, 7);
        assert!(result == expected);
    }
}
