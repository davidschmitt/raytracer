use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub enum LightKind {
    Point,
}

pub struct Light {
    position: Tuple,
    intensity: Color,
    kind: LightKind,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 85
    #[test]
    pub fn should_compare_light() {
        let intensity = Color::new(1, 1, 1);
        let position = Tuple::point(0, 0, 0);
        let light = point(position, intensity);
        assert!(light.position == position);
        assert!(light.intensity == intensity);
    }
}