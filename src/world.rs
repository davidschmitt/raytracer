use crate::color::Color;
use crate::float::Float;
use crate::intersection::Intersections;
use crate::light::Light;
use crate::material::Material;
use crate::ray::Ray;
use crate::shape::Shape;
use crate::transform::Transform;
use crate::tuple::Tuple;
use std::vec::Vec;

pub struct World {
    pub light: Option<Light>,
    pub objects: Vec<Shape>,
}

impl World {
    pub fn new() -> World {
        return World {
            light: None,
            objects: Vec::new(),
        };
    }

    pub fn default() -> World {
        let mut m1 = Material::new();
        m1.color = Color::new(0.6, 1.0, 0.6);
        m1.diffuse = Float::from(0.7);
        m1.specular = Float::from(0.2);
        let mut s1 = Shape::sphere();
        s1.material = m1;
        let mut s2 = Shape::sphere();
        s2.transform = Transform::scaling(0.5, 0.5, 0.5);
        let light = Light::point(Tuple::point(-10, 10, -10), Color::new(1, 1, 1));
        let mut w = World::new();
        w.light = Some(light.clone());
        w.objects.push(s1.clone());
        w.objects.push(s2.clone());
        return w;
    }

    pub fn intersect(self: &World, opray: &Ray) -> Intersections {
        let mut intList = Intersections::new();
        for o in self.objects.iter() {
            let tempList = o.intersect(opray);
            for i in tempList.0.iter() {
                intList.0.push(i.clone());
            }
        }
        intList.sort();
        return intList;
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::Tuple;

    // Page 92
    #[test]
    pub fn should_create_empty_world() {
        let w = World::new();
        assert!(w.objects.len() == 0);
        match w.light {
            None => assert!(true),
            Some(..) => assert!(false),
        }
    }

    // Page 92
    #[test]
    pub fn should_add_things_to_world() {
        let w = World::default();
        match w.light {
            None => assert!(false),
            Some(inner) => assert!(inner == inner),
        }
        assert!(w.objects.contains(&w.objects[0]));
        assert!(w.objects.contains(&w.objects[1]));
    }

    // Page 92
    #[test]
    pub fn should_ray_intersect_world() {
        let w = World::default();
        let r = Ray::new(Tuple::point(0, 0, -5), Tuple::vector(0, 0, 1));
        let xs = w.intersect(&r);
        println!("{:?}", xs);
        assert!(xs.0.len() == 4);

        assert!(xs[0].t == 4);
        assert!(xs[1].t == 4.5);
        assert!(xs[2].t == 5.5);
        assert!(xs[3].t == 6);
    }
}
