use std::vec;

use crate::float::F64IsAbout;
use crate::intersection::{intersection, IntersectionMethods, IntersectionsMethods};
use crate::matrix4::{matrix4, Matrix4, Matrix4Methods};
use crate::shape::{sphere, Shape, ShapeMethods};
use crate::tuple::{point, vector, Tuple, TupleMethods};
use crate::transform;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

pub trait RayMethods {
    fn position(&self, magnitude: f64) -> Tuple;
    fn transform(&self, matrix: &Matrix4) -> Ray;
}

impl RayMethods for Ray {

    fn position(self: &Ray, magnitude: f64) -> Tuple {
        return self.origin.add(&self.direction.multiply(magnitude));
    }

    fn transform(self: &Ray, matrix: &Matrix4) -> Ray {
        return ray(
            &matrix.times_tuple(&self.origin),
            &matrix.times_tuple(&self.direction),
        );
    }
}

pub fn ray(origin: &Tuple, direction: &Tuple) -> Ray {
    return Ray {
        origin: *origin,
        direction: *direction,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 58
    #[test]
    fn should_create_a_ray() {
        let origin = point(1.0, 2.0, 3.0);
        let direction = vector(4.0, 5.0, 6.0);
        let r = ray(&origin, &direction);
        assert!(r.origin.equals(&origin));
        assert!(r.direction.equals(&direction));
    }

    #[test]
    fn should_calculate_position() {
        let origin = point(2.0, 3.0, 4.0);
        let direction = vector(1.0, 0.0, 0.0);
        let r = ray(&origin, &direction);
        let r1 = r.position(0.0);
        assert!(r1.equals(&origin));
        let r2 = r.position(1.0);
        let e2 = point(3.0, 3.0, 4.0);
        assert!(r2.equals(&e2));
        let r3 = r.position(-1.0);
        let e3 = point(1.0, 3.0, 4.0);
        assert!(r3.equals(&e3));
        let r4 = r.position(2.5);
        let e4 = point(4.5, 3.0, 4.0);
        assert!(r4.equals(&e4));
    }

    #[test]
    fn should_intersect_a_sphere_at_two_points() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t.is_about(4.0));
        assert!(xs[1].t.is_about(6.0));
    }

    #[test]
    fn should_intersect_a_sphere_at_tangent_twice() {
        let r = ray(&point(0.0, 1.0, -5.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t.is_about(5.0));
        assert!(xs[1].t.is_about(5.0));
    }

    #[test]
    fn should_miss_a_sphere() {
        let r = ray(&point(0.0, 2.0, -5.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs =s.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn should_intersect_inside_a_sphere() {
        let r = ray(&point(0.0, 0.0, 0.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs =s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t.is_about(-1.0));
        assert!(xs[1].t.is_about(1.0));
    }

    #[test]
    fn should_intersect_behind_a_sphere() {
        let r = ray(&point(0.0, 0.0, 5.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs =s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t.is_about(-6.0));
        assert!(xs[1].t.is_about(-4.0));
    }

    #[test]
    fn should_return_intersections() {
        let s = sphere();
        let i = intersection(3.5, &s);
        assert!(i.t.is_about(3.5));
        assert!(i.s.equals(&s));
    }

    #[test]
    fn should_agregate_intersections() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let xs = [i1, i2];
        assert!(xs.len() == 2);
        assert!(xs[0].t.is_about(1.0));
        assert!(xs[1].t.is_about(2.0));
    }

    #[test]
    fn should_set_object_on_intersection() {
        let r = ray(&point(0.0, 0.0, -5.0), &vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs =s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(s.equals(&xs[0].s));
        assert!(s.equals(&xs[1].s));
    }

    #[test]
    fn should_hit_when_all_t_positive() {
        let s = sphere();
        let i1 = intersection(1.0, &s);
        let i2 = intersection(2.0, &s);
        let mut xs = Vec::new();
        xs.push(i1);
        xs.push(i2);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i.equals(&i1));
            }
        }
    }

    #[test]
    fn should_hit_with_some_negative() {
        let s = sphere();
        let i1 = intersection(-1.0, &s);
        let i2 = intersection(1.0, &s);
        let xs = Vec::from([i2, i1]);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i.equals(&i2));
            }
        }
    }

    #[test]
    fn no_hit_when_all_negative() {
        let s = sphere();
        let i1 = intersection(-2.0, &s);
        let i2 = intersection(-1.0, &s);
        let xs = Vec::from([i2, i1]);
        let i = xs.hit();
        match i {
            None => assert!(true),
            Some(_i) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn hit_always_smallest_nonnegative() {
        let s = sphere();
        let i1 = intersection(5.0, &s);
        let i2 = intersection(7.0, &s);
        let i3 = intersection(-3.0, &s);
        let i4 = intersection(2.0, &s);
        let xs = Vec::from([i1, i2, i3, i4]);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i.equals(&i4));
            }
        }
    }

    // Page 69
    #[test]
    fn translating_a_ray() {
        let r = ray(&point(1.0, 2.0, 3.0), &vector(0.0, 1.0, 0.0));
        let m = transform::translation(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert!(r2.origin.equals(&point(4.0, 6.0, 8.0)));
        assert!(r2.direction.equals(&vector(0.0, 1.0, 0.0)));
    }
}
