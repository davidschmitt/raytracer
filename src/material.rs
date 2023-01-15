use crate::color::Color;
use crate::float::Float;
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
}
