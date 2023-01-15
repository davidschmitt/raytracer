use crate::float::Float;
use crate::intersection::{Intersection, Intersections};
use crate::material::Material;
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
pub struct Shape {
    pub transform: Matrix4,
    pub material: Material,
    pub kind: ShapeKind,
}

#[derive(Clone, Copy, Debug)]
pub enum ShapeKind {
    Sphere { id: i32 },
}

impl PartialEq<ShapeKind> for ShapeKind {
    fn eq(&self, peer: &ShapeKind) -> bool {
        match (self, peer) {
            (ShapeKind::Sphere { id, .. }, ShapeKind::Sphere { .. }) => {
                let selfid = id;
                match peer {
                    ShapeKind::Sphere { id, .. } => {
                        return id == selfid;
                    }
                }
            }
        }
    }
}

impl PartialEq<Shape> for Shape {
    fn eq(&self, peer: &Shape) -> bool {
        return self.kind == peer.kind
            && self.material == peer.material
            && self.transform == peer.transform;
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
        let transform = self.transform.inverse();
        let ray = opray.transform(&transform);
        match self.kind {
            ShapeKind::Sphere { .. } => {
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

    pub fn normal_at<T: AsRef<Tuple>>(&self, at1: T) -> Tuple {
        match self.kind {
            ShapeKind::Sphere { id, .. } => {
                let t = at1.as_ref();
                let object_point = self.transform.inverse() * t;
                let mut world_normal = self.transform.inverse().transpose() * object_point;
                // Page 82 cheat
                world_normal.w = Float::from(0);
                return world_normal.normalize();
            }
        }
    }

    pub fn sphere() -> Shape {
        return Shape {
            material: Material::new(),
            transform: Transform::identity(),
            kind: ShapeKind::Sphere {
                id: shapeid.fetch_add(1, Ordering::Relaxed),
            },
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
        assert!(Transform::identity() == s.transform);
    }

    #[test]
    fn should_override_shape_transform() {
        let mut s = Shape::sphere();
        let t = Transform::translation(2, 3, 4);
        s.transform = t;
        assert!(t == s.transform);
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
        s.transform = Transform::translation(0, 1, 0);
        let n = s.normal_at(Tuple::point(0, 1.70711, -0.70711));
        assert!(n == Tuple::vector(0, 0.70711, -0.70711));
    }

    //  Page 80
    #[test]
    fn should_compute_normal_on_transformed_sphere() {
        let mut s = Shape::sphere();
        let m = Transform::scaling(1, 0.5, 1) * Transform::rotation_z(std::f64::consts::PI / 5.0);
        s.transform = m;
        let at = Tuple::point(0, 2f64.sqrt() / 2.0, 2f64.sqrt() / -2.0);
        let n = s.normal_at(at);
        assert!(n == Tuple::vector(0, 0.97014, -0.24254));
    }

    // Page 85
    #[test]
    fn should_have_default_material() {
        let s = Shape::sphere();
        let m = Material::new();
        assert!(s.material == m);
    }

    // Page 85
    #[test]
    fn should_have_assignable_material() {
        let mut s = Shape::sphere();
        let mut m = Material::new();
        m.ambient = Float::from(1);
        s.material = m;
        assert!(s.material == m);
    }
}
