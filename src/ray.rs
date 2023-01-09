use crate::matrix4::Matrix4;
use crate::tuple::Tuple;
use std::convert::AsRef;

pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl AsRef<Ray> for Ray {
    fn as_ref(&self) -> &Ray {
        return self;
    }
}

impl Ray {
    pub fn new<S: AsRef<Tuple>, T: AsRef<Tuple>>(origin: S, direction: T) -> Ray {
        return Ray {
            origin: *(origin.as_ref()),
            direction: *(direction.as_ref()),
        };
    }

    pub fn position<S: Into<f64>>(self: &Ray, magnitude: S) -> Tuple {
        return self.origin + (self.direction * magnitude.into());
    }

    pub fn transform<M: AsRef<Matrix4>>(self: &Ray, matrix: M) -> Ray {
        let m = matrix.as_ref();
        return Ray::new(m * self.origin, m * self.direction);
    }
}

#[cfg(test)]
mod tests {

    use crate::intersection::{Intersection, Intersections};
    use crate::shape::Shape;
    use crate::transform::Transform;

    use super::*;

    // Page 58
    #[test]
    fn should_create_a_ray() {
        let origin = Tuple::point(1, 2, 3);
        let direction = Tuple::vector(4, 5, 6);
        let r = Ray::new(origin, direction);
        assert!(r.origin == origin);
        assert!(r.direction == direction);
    }

    #[test]
    fn should_calculate_position() {
        let origin = Tuple::point(2, 3, 4);
        let direction = Tuple::vector(1, 0, 0);
        let r = Ray::new(origin, direction);
        let r1 = r.position(0);
        assert!(r1 == origin);
        let r2 = r.position(1);
        let e2 = Tuple::point(3, 3, 4);
        assert!(r2 == e2);
        let r3 = r.position(-1);
        let e3 = Tuple::point(1, 3, 4);
        assert!(r3 == e3);
        let r4 = r.position(2.5);
        let e4 = Tuple::point(4.5, 3, 4);
        assert!(r4 == e4);
    }

    #[test]
    fn should_intersect_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t == 4);
        assert!(xs[1].t == 6);
    }

    #[test]
    fn should_intersect_a_sphere_at_tangent_twice() {
        let r = Ray::new(Tuple::point(0, 1, -5), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t == 5);
        assert!(xs[1].t == 5);
    }

    #[test]
    fn should_miss_a_sphere() {
        let r = Ray::new(Tuple::point(0, 2, -5), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn should_intersect_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0, 0, 0), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t == -1);
        assert!(xs[1].t == 1);
    }

    #[test]
    fn should_intersect_behind_a_sphere() {
        let r = Ray::new(Tuple::point(0, 0, 5), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(xs[0].t == -6);
        assert!(xs[1].t == -4);
    }

    #[test]
    fn should_return_intersections() {
        let s = Shape::sphere();
        let i = Intersection::new(3.5, &s);
        assert!(i.t == 3.5);
        assert!(i.s == s);
    }

    #[test]
    fn should_agregate_intersections() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1, &s);
        let i2 = Intersection::new(2, &s);
        let xs = [i1, i2];
        assert!(xs.len() == 2);
        assert!(xs[0].t == 1);
        assert!(xs[1].t == 2);
    }

    #[test]
    fn should_set_object_on_intersection() {
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
        let s = Shape::sphere();
        let xs = s.intersect(&r);
        assert!(xs.len() == 2);
        assert!(s == xs[0].s);
        assert!(s == xs[1].s);
    }

    #[test]
    fn should_hit_when_all_t_positive() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1, &s);
        let i2 = Intersection::new(2, &s);
        let mut xs = Intersections::new();
        xs.push(i1);
        xs.push(i2);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i == i1);
            }
        }
    }

    #[test]
    fn should_hit_with_some_negative() {
        let s = Shape::sphere();
        let i1 = Intersection::new(-1, s);
        let i2 = Intersection::new(1, s);
        let xs = Intersections::from(&[i2, i1]);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i == i2);
            }
        }
    }

    #[test]
    fn no_hit_when_all_negative() {
        let s = Shape::sphere();
        let i1 = Intersection::new(-2, &s);
        let i2 = Intersection::new(-1, &s);
        let xs = Intersections::from(&[i2, i1]);
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
        let s = Shape::sphere();
        let i1 = Intersection::new(5, &s);
        let i2 = Intersection::new(7, &s);
        let i3 = Intersection::new(-3, &s);
        let i4 = Intersection::new(2, &s);
        let xs = Intersections::from(&[i1, i2, i3, i4]);
        let option = xs.hit();
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(i == i4);
            }
        }
    }

    // Page 69
    #[test]
    fn translating_a_ray() {
        let r = Ray::new(Tuple::point(1, 2, 3), Tuple::vector(0, 1, 0));
        let m = Transform::translation(3, 4, 5);
        let r2 = r.transform(&m);
        assert!(r2.origin == Tuple::point(4, 6, 8));
        assert!(r2.direction == Tuple::vector(0, 1, 0));
    }
}
