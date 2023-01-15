use crate::color::Color;
use crate::float::Float;
use crate::light::Light;
use crate::tuple::Tuple;
use std::cmp::PartialEq;
use std::convert::AsRef;

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub color: Color,
    pub ambient: Float,
    pub diffuse: Float,
    pub shininess: Float,
    pub specular: Float,
}

impl AsRef<Material> for Material {
    fn as_ref(&self) -> &Material {
        return self;
    }
}

impl PartialEq<Material> for Material {
    fn eq(&self, other: &Material) -> bool {
        return self.color == other.color
            && self.ambient == other.ambient
            && self.diffuse == other.diffuse
            && self.shininess == other.shininess
            && self.specular == other.specular;
    }
    fn ne(&self, other: &Material) -> bool {
        return !self.eq(other);
    }
}

impl Material {
    pub fn new() -> Material {
        return Material {
            color: Color::new(1, 1, 1),
            ambient: Float::from(0.1),
            diffuse: Float::from(0.9),
            shininess: Float::from(200),
            specular: Float::from(0.9),
        };
    }

    pub fn lighting<L: AsRef<Light>, P: AsRef<Tuple>, E: AsRef<Tuple>, N: AsRef<Tuple>>(
        self,
        light: L,
        position: P,
        eyev: E,
        normalv: N,
    ) -> Color {
        let effective_color = self.color * light.as_ref().intensity;
        let lightv = (light.as_ref().position - position.as_ref()).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.as_ref().dot(normalv.as_ref());
        let mut diffuse = Color::BLACK;
        let mut specular = Color::BLACK;
        if light_dot_normal < 0 {
            diffuse = Color::BLACK;
            specular = Color::BLACK;
        } else {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = -lightv.as_ref().reflect(normalv.as_ref());
            let reflect_dot_eye = reflectv.as_ref().dot(eyev);
            if reflect_dot_eye <= 0 {
                specular = Color::BLACK;
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.as_ref().intensity * self.specular * factor;
            }
        }

        return ambient + diffuse + specular;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 85
    #[test]
    pub fn should_have_good_material_defaults() {
        let m = Material::new();
        assert!(m.color == Color::new(1, 1, 1));
        assert!(m.ambient == 0.1);
        assert!(m.diffuse == 0.9);
        assert!(m.specular == 0.9);
        assert!(m.shininess == 200);
    }

    // Page 86
    #[test]
    pub fn should_calc_when_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Tuple::point(0, 0, 0);
        let eyev = Tuple::vector(0, 0, -1);
        let normalv = Tuple::vector(0, 0, -1);
        let light = Light::point(Tuple::point(0, 0, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        println!("Color {:?}", result);
        assert!(result == Color::new(1.9, 1.9, 1.9));
    }

    // Page 87
    #[test]
    pub fn shoudl_calc_with_light_offset_45_degrees() {
        let m = Material::new();
        let position = Tuple::point(0, 0, 0);
        let eyev = Tuple::vector(0, 0, -1);
        let normalv = Tuple::vector(0, 0, -1);
        let light = Light::point(Tuple::point(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        assert!(result == Color::new(0.7364, 0.7364, 0.7364));
    }

    // Page 87
    #[test]
    pub fn shoudl_calc_with_eye_in_path_of_reflection() {
        let r = 2.0f64.sqrt() / -2.0;
        let m = Material::new();
        let position = Tuple::point(0, 0, 0);
        let eyev = Tuple::vector(0, r, r);
        let normalv = Tuple::vector(0, 0, -1);
        let light = Light::point(Tuple::point(0, 10, -10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        assert!(result == Color::new(1.6364, 1.6364, 1.6364));
    }

    // Page 88
    #[test]
    pub fn shoudl_calc_with_light_behind_surface() {
        let m = Material::new();
        let position = Tuple::point(0, 0, 0);
        let eyev = Tuple::vector(0, 0, -1);
        let normalv = Tuple::vector(0, 0, -1);
        let light = Light::point(Tuple::point(0, 0, 10), Color::new(1, 1, 1));
        let result = m.lighting(light, position, eyev, normalv);
        assert!(result == Color::new(0.1, 0.1, 0.1));
    }
}
