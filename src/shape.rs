use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use crate::intersection;
use crate::matrix4::{self, Matrix4Methods};
use crate::tuple::{point, vector, Tuple, TupleMethods};
use crate::ray::Ray;
use crate::ray::RayMethods;
use crate::transform;
static shapeid: AtomicI32 = AtomicI32::new(1);

pub const PI: f64 = std::f64::consts::PI;

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere { id: i32, transform: matrix4::Matrix4 },
}

pub trait ShapeMethods {
    fn equals(&self, peer: &Shape) -> bool;
    fn intersect(&self, ray: &Ray) -> Vec<intersection::Intersection>;
    fn set_transform(&mut self, mfour: &matrix4::Matrix4);
    fn get_transform(&self) -> matrix4::Matrix4;
    fn normal_at(&self, point: &Tuple) -> Tuple;
}

impl ShapeMethods for Shape {
    fn equals(self: &Shape, peer: &Shape) -> bool {
        match (self, peer) {
            (Shape::Sphere { id, .. }, Shape::Sphere { .. }) => {
                let selfid = id;
                match (peer) {
                    Shape::Sphere { id, .. } => return selfid == id,
                }
            }
            _ => false,
        }
    }

    fn intersect(self: &Shape, opray: &Ray) -> Vec<intersection::Intersection> {
        let transform = self.get_transform().inverse();
        let ray = opray.transform(&transform);
        match self {
            Shape::Sphere { .. } => {
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

    fn set_transform(&mut self, mfour: &matrix4::Matrix4) {
        match self {
            Shape::Sphere { ref mut transform, .. } => {
                *transform = *mfour;
            }
        }
    }


    fn get_transform(&self) -> matrix4::Matrix4 {
        match self {
            Shape::Sphere { transform, .. } => {
                return *transform;
            }
        }
    }

    fn normal_at(&self, at1: &Tuple) -> Tuple {
        match self {
            Shape::Sphere { transform, .. } =>  {
                let at2 = transform.inverse().times_tuple(at1);
                let mut dif = at2.subtract(&point(0.0, 0.0, 0.0));
                dif.w = 0.0;
                let mut dif2 = transform.times_tuple(&dif);
                dif2.w = 0.0;
                println!("at1: {:?}, at2: {:?}, dif1: {:?}, dif2: {:?}", at1, at2, dif, dif2);
                return dif2.normalize();
            }
        }
    }
}

pub fn sphere() -> Shape {
    return Shape::Sphere {
        id: (shapeid.fetch_add(1, Ordering::Relaxed)),
        transform: matrix4::IDENTITY
    };
}

#[cfg(test)]
mod tests {

    use crate::transform::{translation, scaling, rotation_z};

    use super::*;

    // Page 69
    #[test]
    fn should_have_default_shape_transform() {
        let s = sphere();
        assert!(matrix4::IDENTITY.equals(&s.get_transform()));
    }

    #[test]
    fn should_override_shape_transform() {
        let mut s = sphere();
        let t = transform::translation(2.0, 3.0, 4.0);
        s.set_transform(&t);
        assert!(t.equals(&s.get_transform()));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_x() {
        let s = sphere();
        let n = s.normal_at(&point(1.0, 0.0, 0.0));
        assert!(n.equals(&vector(1.0, 0.0, 0.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_y() {
        let s = sphere();
        let n = s.normal_at(&point(0.0, 1.0, 0.0));
        assert!(n.equals(&vector(0.0, 1.0, 0.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_on_z() {
        let s = sphere();
        let n = s.normal_at(&point(0.0, 0.0, 1.0));
        assert!(n.equals(&vector(0.0, 0.0, 1.0)));
    }

    //  Page 78
    #[test]
    fn should_calc_sphere_normal_nonaxial() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = sphere();
        let n = s.normal_at(&point(f, f, f));
        assert!(n.equals(&vector(f, f, f)));
    }

    //  Page 78
    #[test]
    fn should_normalize_normals() {
        // (Square root of 3) divided by 3
        let f = 3.0_f64.sqrt() / 3.0;
        let s = sphere();
        let n = s.normal_at(&point(f, f, f));
        assert!(n.equals(&n.normalize()));
    }

    //  Page 80
    #[test]
    fn should_compute_translated_sphere_normal() {
        let mut s = sphere();
        s.set_transform(&translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&point(0.0, 1.70711, -0.70711));
        assert!(n.equals(&vector(0.0, 0.70711, -0.70711)));
    }

    //  Page 80
    #[test]
    fn should_compute_normal_on_transformed_sphere() {
        let mut s = sphere();
        let m = scaling(1.0, 0.5, 1.0).multiply(&rotation_z(std::f64::consts::PI / 5.0));
        s.set_transform(&m);
        let at = point(0.0, 2f64.sqrt() / 2.0, 2f64.sqrt() / - 2.0);
        let n = s.normal_at(&at);
        assert!(n.equals(&vector(0.0, 0.97014, -0.24254)));
    }
}
