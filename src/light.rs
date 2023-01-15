use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub enum LightKind {
    Point,
}

pub struct Light {
    pub position: Tuple,
    pub intensity: Color,
    pub kind: LightKind,
}

impl AsRef<Light> for Light {
    fn as_ref(&self) -> &Light {
        return self;
    }
}

impl Light {
    pub fn point<P: AsRef<Tuple>, I: AsRef<Color>>(position: P, intensity: I) -> Light {
        return Light {
            position: *position.as_ref(),
            intensity: *intensity.as_ref(),
            kind: LightKind::Point,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 85
    #[test]
    pub fn should_compare_light() {
        let intensity = Color::new(1, 1, 1);
        let position = Tuple::point(0, 0, 0);
        let light = Light::point(position, intensity);
        assert!(light.position == position);
        assert!(light.intensity == intensity);
    }
}
