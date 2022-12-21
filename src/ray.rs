use std::vec;

use crate::float;
use crate::intersection;
use crate::matrix4;
use crate::shape;
use crate::shape::Shape;
use crate::transform;
use crate::tuple;

pub struct Ray {
    pub origin: tuple::Tuple,
    pub direction: tuple::Tuple,
}

pub fn ray(origin: &tuple::Tuple, direction: &tuple::Tuple) -> Ray {
    return Ray {
        origin: *origin,
        direction: *direction,
    };
}

pub fn position(ray: &Ray, magnitude: f64) -> tuple::Tuple {
    return tuple::add(&ray.origin, &tuple::multiply(&ray.direction, magnitude));
}

pub fn spheretoray(ray: &Ray) -> tuple::Tuple {
    return tuple::subtract(&ray.origin, &tuple::point(0.0, 0.0, 0.0));
}

pub fn intersect(shape: &Shape, ray: &Ray) -> Vec<intersection::Intersection> {
    match shape {
        Shape::Sphere { id } => {
            let d = spheretoray(&ray);
            let a = tuple::dot(&ray.direction, &ray.direction);
            let b = 2.0 * tuple::dot(&ray.direction, &d);
            let c = tuple::dot(&d, &d) - 1.0;

            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                return Vec::new();
            }

            let t1 = (-b - (discriminant.sqrt())) / (2.0 * a);
            let t2 = (-b + (discriminant.sqrt())) / (2.0 * a);

            let mut vec = Vec::new();
            vec.push(intersection::intersection(t1, shape));
            vec.push(intersection::intersection(t2, shape));
            return vec;
        }
    }
}

pub fn hit(intvec: &Vec<intersection::Intersection>) -> Option<intersection::Intersection> {
    let mut closest: Option<intersection::Intersection> = None;
    for i in intvec {
        if i.t >= 0.0 {
            match closest {
                None => {
                    closest = Some(*i);
                }
                Some(thing) => {
                    if i.t < thing.t {
                        closest = Some(*i)
                    }
                }
            }
        }
    }
    return closest;
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 58
    #[test]
    fn should_create_a_ray() {
        let origin = tuple::point(1.0, 2.0, 3.0);
        let direction = tuple::vector(4.0, 5.0, 6.0);
        let r = ray(&origin, &direction);
        assert!(tuple::equals(&r.origin, &origin));
        assert!(tuple::equals(&r.direction, &direction));
    }

    #[test]
    fn should_calculate_position() {
        let origin = tuple::point(2.0, 3.0, 4.0);
        let direction = tuple::vector(1.0, 0.0, 0.0);
        let r = ray(&origin, &direction);
        let r1 = position(&r, 0.0);
        assert!(tuple::equals(&r1, &origin));
        let r2 = position(&r, 1.0);
        let e2 = tuple::point(3.0, 3.0, 4.0);
        assert!(tuple::equals(&r2, &e2));
        let r3 = position(&r, -1.0);
        let e3 = tuple::point(1.0, 3.0, 4.0);
        assert!(tuple::equals(&r3, &e3));
        let r4 = position(&r, 2.5);
        let e4 = tuple::point(4.5, 3.0, 4.0);
        assert!(tuple::equals(&r4, &e4));
    }

    #[test]
    fn should_intersect_a_sphere_at_two_points() {
        let r = ray(&tuple::point(0.0, 0.0, -5.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 2);
        assert!(float::equals(xs[0].t, 4.0));
        assert!(float::equals(xs[1].t, 6.0));
    }

    #[test]
    fn should_intersect_a_sphere_at_tangent_twice() {
        let r = ray(&tuple::point(0.0, 1.0, -5.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 2);
        assert!(float::equals(xs[0].t, 5.0));
        assert!(float::equals(xs[1].t, 5.0));
    }

    #[test]
    fn should_miss_a_sphere() {
        let r = ray(&tuple::point(0.0, 2.0, -5.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 0);
    }

    #[test]
    fn should_intersect_inside_a_sphere() {
        let r = ray(&tuple::point(0.0, 0.0, 0.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 2);
        assert!(float::equals(xs[0].t, -1.0));
        assert!(float::equals(xs[1].t, 1.0));
    }

    #[test]
    fn should_intersect_behind_a_sphere() {
        let r = ray(&tuple::point(0.0, 0.0, 5.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 2);
        assert!(float::equals(xs[0].t, -6.0));
        assert!(float::equals(xs[1].t, -4.0));
    }

    #[test]
    fn should_return_intersections() {
        let s = shape::sphere();
        let i = intersection::intersection(3.5, &s);
        assert!(float::equals(i.t, 3.5));
        assert!(shape::equals(&i.s, &s));
    }

    #[test]
    fn should_agregate_intersections() {
        let s = shape::sphere();
        let i1 = intersection::intersection(1.0, &s);
        let i2 = intersection::intersection(2.0, &s);
        let xs = [i1, i2];
        assert!(xs.len() == 2);
        assert!(float::equals(xs[0].t, 1.0));
        assert!(float::equals(xs[1].t, 2.0));
    }

    #[test]
    fn should_set_object_on_intersection() {
        let r = ray(&tuple::point(0.0, 0.0, -5.0), &tuple::vector(0.0, 0.0, 1.0));
        let s = shape::sphere();
        let xs = intersect(&s, &r);
        assert!(xs.len() == 2);
        assert!(shape::equals(&s, &xs[0].s));
        assert!(shape::equals(&s, &xs[1].s));
    }

    #[test]
    fn should_hit_when_all_t_positive() {
        let s = shape::sphere();
        let i1 = intersection::intersection(1.0, &s);
        let i2 = intersection::intersection(2.0, &s);
        let mut xs = Vec::new();
        xs.push(i1);
        xs.push(i2);
        let option = hit(&xs);
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(intersection::equals(&i, &i1));
            }
        }
    }

    #[test]
    fn should_hit_with_some_negative() {
        let s = shape::sphere();
        let i1 = intersection::intersection(-1.0, &s);
        let i2 = intersection::intersection(1.0, &s);
        let xs = intersection::intersections(&[i2, i1]);
        let option = hit(&xs);
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(intersection::equals(&i, &i2));
            }
        }
    }

    #[test]
    fn no_hit_when_all_negative() {
        let s = shape::sphere();
        let i1 = intersection::intersection(-2.0, &s);
        let i2 = intersection::intersection(-1.0, &s);
        let xs = intersection::intersections(&[i2, i1]);
        let i = hit(&xs);
        match i {
            None => assert!(true),
            Some(_i) => {
                assert!(false);
            }
        }
    }

    #[test]
    fn hit_always_smallest_nonnegative() {
        let s = shape::sphere();
        let i1 = intersection::intersection(5.0, &s);
        let i2 = intersection::intersection(7.0, &s);
        let i3 = intersection::intersection(-3.0, &s);
        let i4 = intersection::intersection(2.0, &s);
        let xs = intersection::intersections(&[i1, i2, i3, i4]);
        let option = hit(&xs);
        match option {
            None => assert!(false),
            Some(i) => {
                assert!(intersection::equals(&i, &i4));
            }
        }
    }

    // Page 69
    #[test]
    fn translating_a_ray() {
        let r = ray(&tuple::point(1.0, 2.0, 3.0), &tuple::vector(0.0, 1.0, 0.0));
        let m = transform::translation(3.0, 4.0, 5.0);
        let r2 = transform::transform(&r, &m);
        assert!(tuple::equals(&r2.origin, &tuple::point(4.0, 6.0, 8.0)));
        assert!(tuple::equals(&r2.direction, &tuple::vector(0.0, 1.0, 0.0)));
    }
}
