use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use crate::intersection;
use crate::matrix4;
use crate::ray::Ray;
use crate::transform;
use crate::tuple::{point, vector, TupleMethods};
static shapeid: AtomicI32 = AtomicI32::new(1);

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere { id: i32 },
}

pub trait ShapeMethods {
    fn equals(&self, peer: &Shape) -> bool;
    fn intersect(&self, ray: &Ray) -> Vec<intersection::Intersection>;
}

impl ShapeMethods for Shape {
    fn equals(self: &Shape, peer: &Shape) -> bool {
        match (self, peer) {
            (Shape::Sphere { id }, Shape::Sphere { .. }) => {
                let selfid = id;
                match (peer) {
                    Shape::Sphere { id } => return selfid == id,
                }
            }
            _ => false,
        }
    }

    fn intersect(self: &Shape, ray: &Ray) -> Vec<intersection::Intersection> {
        match self {
            Shape::Sphere { id } => {
                let d = ray.origin.subtract(&point(0.0, 0.0, 0.0));
                let a = ray.direction.dot(&ray.direction);
                let b = 2.0 * ray.direction.dot(&d);
                let c = d.dot(&d) - 1.0;

                let discriminant = b.powi(2) - 4.0 * a * c;

                if discriminant < 0.0 {
                    return Vec::new();
                }

                let t1 = (-b - (discriminant.sqrt())) / (2.0 * a);
                let t2 = (-b + (discriminant.sqrt())) / (2.0 * a);

                let mut vec = Vec::new();
                vec.push(intersection::intersection(t1, self));
                vec.push(intersection::intersection(t2, self));
                return vec;
            }
        }
    }
}

pub fn sphere() -> Shape {
    return Shape::Sphere {
        id: (shapeid.fetch_add(1, Ordering::Relaxed)),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // Page 69
    #[test]
    fn should_have_default_shape_transform() {
        let s = sphere();
        assert!(matrix4::IDENTITY.equals(&s.transform));
    }

    #[test]
    fn should_override_shape_transform() {
        let mut s = sphere();
        let t = transform::translation(2.0, 3.0, 4.0);
        set_transform(&s, &t);
        assert!(matrix4::equals(&t, &s.transform));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_x() {
        let s = sphere();
        let n = normal_at(&s, &point(1.0, 0.0, 0.0));
        assert!(tuple::equals(&n, &vector(1.0, 0.0, 0.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_y() {
        let s = sphere();
        let n = normal_at(&s, &point(0.0, 1.0, 0.0));
        assert!(tuple::equals(&n, &vector(0.0, 1.0, 0.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_z() {
        let s = sphere();
        let n = normal_at(&s, &point(0.0, 0.0, 1.0));
        assert!(tuple::equals(&n, &vector(0.0, 0.0, 1.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_nonaxial() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = sphere();
        let n = normal_at(&s, &point(f, f, f));
        assert!(tuple::equals(&n, &vector(f, f, f)));
    }

    //  Page 78
    #[test]
    fn should_normalize_normals() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = sphere();
        let n = normal_at(&s, &point(f, f, f));
        assert!(tuple::equals(&n, &tuple::normalize(&n)));
    }

    //  Page 80
    #[test]
    fn should_compute_translated_sphere_normal() {
        let s = sphere();
    }
    */
}
