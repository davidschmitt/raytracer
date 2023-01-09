use crate::float::Float;
use crate::intersection::{Intersection, Intersections};
use crate::matrix4::Matrix4;
use crate::ray::Ray;
use crate::transform::Transform;
use crate::tuple::Tuple;
use std::cmp::PartialEq;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
static shapeid: AtomicI32 = AtomicI32::new(1);

pub const PI: f64 = std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere { id: i32, transform: Matrix4 },
}

impl PartialEq<Shape> for Shape {
    fn eq(&self, peer: &Shape) -> bool {
        match (self, peer) {
            (Shape::Sphere { id, .. }, Shape::Sphere { .. }) => {
                let selfid = id;
                match peer {
                    Shape::Sphere { id, .. } => return selfid == id,
                }
            }
            _ => false,
        }
    }
    fn ne(&self, other: &Shape) -> bool {
        return !self.eq(other);
    }
}

impl AsRef<Shape> for Shape {
    fn as_ref(&self) -> &Shape {
        return self;
    }
}

impl Shape {
    pub fn intersect(self: &Shape, opray: &Ray) -> Intersections {
        let transform = self.get_transform().inverse();
        let ray = opray.transform(&transform);
        match self {
            Shape::Sphere { .. } => {
                let d = ray.origin - Tuple::point(0, 0, 0);
                let a = ray.direction.dot(ray.direction);
                let b = ray.direction.dot(d) * 2;
                let c = d.dot(d) - 1;

                let discriminant = b.powi(2) - a * c * 4;

                let mut vec = Intersections::new();
                if discriminant < 0 {
                    return vec;
                }

                let t1 = (-b - (discriminant.sqrt())) / (a * 2);
                let t2 = (-b + (discriminant.sqrt())) / (a * 2);

                vec.push(Intersection::new(t1, self));
                vec.push(Intersection::new(t2, self));
                return vec;
            }
        }
    }

    pub fn set_transform<M: AsRef<Matrix4>>(&mut self, mfour: M) {
        match self {
            Shape::Sphere {
                ref mut transform, ..
            } => {
                *transform = *(mfour.as_ref());
            }
        }
    }

    pub fn get_transform(&self) -> Matrix4 {
        match self {
            Shape::Sphere { transform, .. } => {
                return *transform;
            }
        }
    }

    pub fn normal_at<T: AsRef<Tuple>>(&self, at1: T) -> Tuple {
        match self {
            Shape::Sphere { transform, .. } => {
                let at2 = transform.inverse() * at1.as_ref();
                let mut dif = at2 - Tuple::point(0, 0, 0);
                dif.w = Float::from(0);
                let mut dif2 = transform * dif;
                dif2.w = Float::from(0);
                println!(
                    "at1: {:?}, at2: {:?}, dif1: {:?}, dif2: {:?}",
                    at1.as_ref(),
                    at2,
                    dif,
                    dif2
                );
                return dif2.normalize();
            }
        }
    }

    pub fn sphere() -> Shape {
        return Shape::Sphere {
            id: (shapeid.fetch_add(1, Ordering::Relaxed)),
            transform: Transform::identity(),
        };
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::transform::Transform;

    // Page 69
    #[test]
    fn should_have_default_shape_transform() {
        let s = Shape::sphere();
        assert!(Transform::identity() == s.get_transform());
    }

    #[test]
    fn should_override_shape_transform() {
        let mut s = Shape::sphere();
        let t = Transform::translation(2, 3, 4);
        s.set_transform(&t);
        assert!(t == s.get_transform());
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_x() {
        let s = Shape::sphere();
        let n = s.normal_at(Tuple::point(1, 0, 0.0));
        assert!(n == Tuple::vector(1.0, 0.0, 0.0));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_y() {
        let s = Shape::sphere();
        let n = s.normal_at(Tuple::point(0, 1, 0));
        assert!(n == Tuple::vector(0, 1, 0));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_z() {
        let s = Shape::sphere();
        let n = s.normal_at(Tuple::point(0, 0, 1));
        assert!(n == Tuple::vector(0, 0, 1));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_nonaxial() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = Shape::sphere();
        let n = s.normal_at(Tuple::point(f, f, f));
        assert!(n == Tuple::vector(f, f, f));
    }

    //  Page 78
    #[test]
    fn should_normalize_normals() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = Shape::sphere();
        let n = s.normal_at(Tuple::point(f, f, f));
        assert!(n == n.normalize());
    }

    //  Page 80
    #[test]
    fn should_compute_translated_sphere_normal() {
        let mut s = Shape::sphere();
        s.set_transform(Transform::translation(0, 1, 0));
        let n = s.normal_at(Tuple::point(0, 1.70711, -0.70711));
        assert!(n == Tuple::vector(0, 0.70711, -0.70711));
    }

    //  Page 80
    #[test]
    fn should_compute_normal_on_transformed_sphere() {
        let mut s = Shape::sphere();
        let m = Transform::scaling(1, 0.5, 1) * Transform::rotation_z(std::f64::consts::PI / 5.0);
        s.set_transform(m);
        let at = Tuple::point(0, 2f64.sqrt() / 2.0, 2f64.sqrt() / -2.0);
        let n = s.normal_at(at);
        assert!(n == Tuple::vector(0, 0.97014, -0.24254));
    }
}
