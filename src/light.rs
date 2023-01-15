use crate::color::Color;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug)]
pub enum Light {
    point { position: Tuple, intensity: Color },
}
