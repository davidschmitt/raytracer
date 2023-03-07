use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub enum LightKind {
    Point,
}

impl PartialEq<LightKind> for LightKind {
    fn eq(&self, peer: &LightKind) -> bool {
        match (self, peer) {
            (LightKind::Point { .. }, LightKind::Point { .. }) => return true,
        }
        return false;
    }
}

#[derive(Clone, Copy, Debug)]
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

impl PartialEq<Light> for Light {
    fn eq(&self, rhs: &Light) -> bool {
        return self.position == rhs.position
            && self.intensity == rhs.intensity
            && self.kind == rhs.kind;
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
