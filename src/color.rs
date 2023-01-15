use crate::float::Float;
use std::cmp::PartialEq;
use std::convert::AsRef;
use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: Float,
    pub green: Float,
    pub blue: Float,
}

impl AsRef<Color> for Color {
    fn as_ref(&self) -> &Color {
        return self;
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, rhs: &Color) -> bool {
        return self.red == rhs.red && self.green == rhs.green && self.blue == rhs.blue;
    }
    fn ne(&self, rhs: &Color) -> bool {
        return !self.eq(rhs);
    }
}

impl<S: AsRef<Color>> Add<S> for Color {
    type Output = Color;
    fn add(self, rhs: S) -> Self::Output {
        let peer = rhs.as_ref();
        Color {
            red: self.red + peer.red,
            green: self.green + peer.green,
            blue: self.blue + peer.blue,
        }
    }
}

impl<S: AsRef<Color>> Sub<S> for Color {
    type Output = Color;
    fn sub(self, rhs: S) -> Self::Output {
        let peer = rhs.as_ref();
        Color {
            red: self.red - peer.red,
            green: self.green - peer.green,
            blue: self.blue - peer.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<&Color> for Color {
    type Output = Color;
    fn mul(self, rhs: &Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl<S: Into<f64>> Mul<S> for Color {
    type Output = Color;
    fn mul(self, rhs: S) -> Self::Output {
        let scale = rhs.into();
        Color {
            red: self.red * scale,
            green: self.green * scale,
            blue: self.blue * scale,
        }
    }
}

pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

impl<S: AsRef<Color>> From<S> for Pixel {
    fn from(color: S) -> Self {
        let x = color.as_ref();
        return Pixel {
            red: normalize(x.red),
            green: normalize(x.green),
            blue: normalize(x.blue),
        };
    }
}

impl Color {
    pub const BLACK: Color = Color {
        red: Float(0.0),
        green: Float(0.0),
        blue: Float(0.0),
    };

    pub const WHITE: Color = Color {
        red: Float(1.0),
        green: Float(1.0),
        blue: Float(1.0),
    };

    pub fn new<R: Into<f64>, G: Into<f64>, B: Into<f64>>(red: R, green: G, blue: B) -> Color {
        return Color {
            red: Float(red.into()),
            green: Float(green.into()),
            blue: Float(blue.into()),
        };
    }
}

fn normalize(value: Float) -> i32 {
    let max = 255;
    if value < 0.0 {
        return 0;
    } else if value > 1.0 {
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
        let c = Color::new(-0.5, 0.4, 1.7);
        assert!(c.red == -0.5);
        assert!(c.green == 0.4);
        assert!(c.blue == 1.7);
    }

    // Page 17
    #[test]
    fn should_compare_colors() {
        let expected = Color::new(0.0, 0.0, 0.0);
        for red in 0..2 {
            for green in 0..2 {
                for blue in 0..2 {
                    let value = Color::new(f64::from(red), f64::from(green), f64::from(blue));
                    if red + green + blue > 0 {
                        assert!(expected != value);
                    } else {
                        assert!(expected == value);
                    }
                }
            }
        }
    }

    // Page 17
    #[test]
    fn should_add_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);
        assert!(result == expected);
    }

    // Page 17
    #[test]
    fn should_subtract_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let result = c1 - c2;
        let expected = Color::new(0.2, 0.5, 0.5);
        assert!(result == expected);
    }

    // Page 17
    #[test]
    fn should_multiply_colors() {
        let c1 = Color::new(0.2, 0.3, 0.4);
        let result = c1 * 2.0;
        let expected = Color::new(0.4, 0.6, 0.8);
        assert!(result == expected);
    }

    // Page 17
    #[test]
    fn should_calculate_hadamard_product() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let result = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);
        assert!(result == expected);
    }
}
