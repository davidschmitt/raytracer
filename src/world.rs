use crate::color::Color;
use crate::float::Float;
use crate::light::Light;
use crate::material::Material;
use crate::shape::Shape;
use crate::transform::Transform;
use std::vec::Vec;

pub struct World {
    pub light: Option<Light>,
    pub objects: Vec<Shape>,
}

#[cfg(test)]
mod tests {

    /*
    // Page 92
    #[test]
    pub fn should_create_empty_world() {
        let w = World::new();
        assert!(w.objects.length() == 0);
        match w {
            None => assert!(true),
            Some(light)=> assert!(false)
        }
    }

    // Page 92
    #[test]
    pub fn should_add_things_to_world() {
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
        w.light = light;
        w.objects.push(s1);
        w.objects.push(s2);

        assert!(w.light == light);
        assert!(w.objects.contains(s1));
        assert!(w.objects.contains(s2));
    }
    */
}
