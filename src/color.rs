use crate::float::F64IsAbout;
use std::ops::{Mul, Sub, Add};
use std::cmp::{PartialEq};
use std::convert::AsRef;
use std::convert::From;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        return self;
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        return self.equals(other);
    }
    fn ne(&self, other: &Color) -> bool {
        return !self.eq(other);
    }
}

impl Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        return self.multiply(rhs);
    }
}

impl Mul<&Color> for Color {
    type Output = Color;

    fn mul(self, rhs: &Color) -> Self::Output {
        return self.multiply(rhs);
    }
}

impl Sub<Color> for Color {
    type Output = Color;

    fn sub(self, rhs: Color) -> Self::Output {
        return self.subtract(rhs);
    }
}

impl Sub<&Color> for Color {
    type Output = Color;

    fn sub(self, rhs: &Color) -> Self::Output {
        return self.subtract(rhs);
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        return self.sum(rhs);
    }
}

impl Add<&Color> for Color {
    type Output = Color;

    fn add(self, rhs: &Color) -> Self::Output {
        return self.sum(rhs);
    }
}

pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl From<&Color> for Pixel {
    fn from(color: &Color) -> Self {
        return Pixel {
            red: normalize(color.red, 255),
            green: normalize(color.green, 255),
            blue: normalize(color.blue, 255),
        };
    }
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    return Color { red, green, blue };
}

pub trait ColorMethods {
    fn equals<T: AsRef<Color>>(&self, peer: T) -> bool;
    fn sum<T: AsRef<Color>>(&self, peer: T) -> Color; 
    fn subtract<T: AsRef<Color>>(&self, peer: T) -> Color;
    fn scale(&self, scalar: f64) -> Color;
    fn multiply<T: AsRef<Color>>(&self, peer: T) -> Color;
}

impl ColorMethods for Color {

    fn equals<T: AsRef<Color>>(self: &Color, other: T) -> bool {
        let peer = other.as_ref();
        return self.red.equals(peer.red)
            && self.green.equals(peer.green)
            && self.blue.equals(peer.blue);
    }

    fn sum<T: AsRef<Color>>(self: &Self, other: T) -> Color {
        let peer = other.as_ref();
        Color {
            red: self.red + peer.red,
            green: self.green + peer.green,
            blue: self.blue + peer.blue,
        }
    }

    fn subtract<T: AsRef<Color>>(self: &Color, other: T) -> Color {
        let peer = other.as_ref();
        Color {
            red: self.red - peer.red,
            green: self.green - peer.green,
            blue: self.blue - peer.blue,
        }
    }

    fn scale(self: &Color, scalar: f64) -> Color {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }

    fn multiply<T: AsRef<Color>>(self: &Color, other: T) -> Color {
        let peer = other.as_ref();
        Color {
            red: self.red * peer.red,
            green: self.green * peer.green,
            blue: self.blue * peer.blue,
        }
    }
}

fn normalize(value: f64, max: i32) -> i32 {
    if value < 0.0 {
        return 0;
    } else if value> 1.0 {
        return max;
    } else {
        return (value * max as f64).round() as i32;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    // Page 16
    #[test]
    fn should_construct_colors() {
        let c = color(-0.5, 0.4, 1.7);
        assert!(c.red.equals(-0.5));
        assert!(c.green.equals(0.4));
        assert!(c.blue.equals(1.7));
    }

    // Page 17
    #[test]
    fn should_compare_colors() {
        let expected = color(0.0, 0.0, 0.0);
        for red in 0..2 {
            for green in 0..2 {
                for blue in 0..2 {
                    let value = color(f64::from(red), f64::from(green), f64::from(blue));
                    if red + green + blue > 0 {
                        assert!(!&expected.equals(&value));
                    } else {
                        assert!(expected.equals(&value));
                    }
                }
            }
        }
    }

    // Page 17
    #[test]
    fn should_add_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1.add(c2);
        let expected = color(1.6, 0.7, 1.0);
        assert!(result == expected);
    }

    // Page 17
    #[test]
    fn should_subtract_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = c1.subtract(&c2);
        let expected = color(0.2, 0.5, 0.5);
        assert!(result.equals(&expected));
    }

    // Page 17
    #[test]
    fn should_multiply_colors() {
        let c1 = color(0.2, 0.3, 0.4);
        let result = c1.scale(2.0);
        let expected = color(0.4, 0.6, 0.8);
        assert!(result.equals(&expected));
    }

    // Page 17
    #[test]
    fn should_calculate_hadamard_product() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        let result = c1 * c2;
        let expected = color(0.9, 0.2, 0.04);
        assert!(result == expected);
    }
}
