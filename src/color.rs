use crate::float::F64IsAbout;

#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub struct Pixel {
    pub red: i32,
    pub green: i32,
    pub blue: i32,
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    return Color { red, green, blue };
}

pub trait ColorMethods {
    fn equals(&self, peer: &Color) -> bool;
    fn add(&self, peer: &Color) -> Color;
    fn subtract(&self, peer: &Color) -> Color;
    fn multiply(&self, scalar: f64) -> Color;
    fn product(&self, peer: &Color) -> Color;
    fn to_pixel(&self, max: i32) -> Pixel;
}

impl ColorMethods for Color {

    fn equals(self: &Color, peer: &Color) -> bool {
        return self.red.is_about(peer.red)
            && self.green.is_about(peer.green)
            && self.blue.is_about(peer.blue);
    }

    fn add(self: &Color, peer: &Color) -> Color {
        Color {
            red: self.red + peer.red,
            green: self.green + peer.green,
            blue: self.blue + peer.blue,
        }
    }

    fn subtract(self: &Color, peer: &Color) -> Color {
        Color {
            red: self.red - peer.red,
            green: self.green - peer.green,
            blue: self.blue - peer.blue,
        }
    }

    fn multiply(self: &Color, scalar: f64) -> Color {
        Color {
            red: self.red * scalar,
            green: self.green * scalar,
            blue: self.blue * scalar,
        }
    }

    fn product(self: &Color, peer: &Color) -> Color {
        Color {
            red: self.red * peer.red,
            green: self.green * peer.green,
            blue: self.blue * peer.blue,
        }
    }

    fn to_pixel(self: &Color, max: i32) -> Pixel {
        return Pixel {
            red: normalize(self.red, max),
            green: normalize(self.green, max),
            blue: normalize(self.blue, max),
        };
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
        assert!(c.red.is_about(-0.5));
        assert!(c.green.is_about(0.4));
        assert!(c.blue.is_about(1.7));
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
        let result = c1.add(&c2);
        let expected = color(1.6, 0.7, 1.0);
        assert!(result.equals(&expected));
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
        let result = c1.multiply(2.0);
        let expected = color(0.4, 0.6, 0.8);
        assert!(result.equals(&expected));
    }

    // Page 17
    #[test]
    fn should_calculate_hadamard_product() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        let result = c1.product(&c2);
        let expected = color(0.9, 0.2, 0.04);
        assert!(result.equals(&expected));
    }
}
