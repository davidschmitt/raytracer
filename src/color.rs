use crate::float;
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

pub fn color(red: f64, green: f64, blue: f64) -> Color {
    return Color { red, green, blue };
}

pub fn equals(a: &Color, b: &Color) -> bool {
    return float::equals(a.red, b.red)
        && float::equals(a.green, b.green)
        && float::equals(a.blue, b.blue);
}

pub fn add(a: &Color, b: &Color) -> Color {
    Color {
        red: a.red + b.red,
        green: a.green + b.green,
        blue: a.blue + b.blue,
    }
}

pub fn subtract(a: &Color, b: &Color) -> Color {
    Color {
        red: a.red - b.red,
        green: a.green - b.green,
        blue: a.blue - b.blue,
    }
}

pub fn multiply(col: &Color, scalar: f64) -> Color {
    Color {
        red: col.red * scalar,
        green: col.green * scalar,
        blue: col.blue * scalar,
    }
}

pub fn product(a: &Color, b: &Color) -> Color {
    Color {
        red: a.red * b.red,
        green: a.green * b.green,
        blue: a.blue * b.blue,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 16
    #[test]
    fn should_construct_colors() {
        let c = color(-0.5, 0.4, 1.7);
        assert!(float::equals(c.red, -0.5));
        assert!(float::equals(c.green, 0.4));
        assert!(float::equals(c.blue, 1.7));
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
                        assert!(!equals(&expected, &value));
                    } else {
                        assert!(equals(&expected, &value));
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
        let result = add(&c1, &c2);
        let expected = color(1.6, 0.7, 1.0);
        assert!(equals(&result, &expected));
    }

    // Page 17
    #[test]
    fn should_subtract_colors() {
        let c1 = color(0.9, 0.6, 0.75);
        let c2 = color(0.7, 0.1, 0.25);
        let result = subtract(&c1, &c2);
        let expected = color(0.2, 0.5, 0.5);
        assert!(equals(&result, &expected));
    }

    // Page 17
    #[test]
    fn should_multiply_colors() {
        let c1 = color(0.2, 0.3, 0.4);
        let result = multiply(&c1, 2.0);
        let expected = color(0.4, 0.6, 0.8);
        assert!(equals(&result, &expected));
    }

    // Page 17
    #[test]
    fn should_calculate_hadamard_product() {
        let c1 = color(1.0, 0.2, 0.4);
        let c2 = color(0.9, 1.0, 0.1);
        let result = product(&c1, &c2);
        let expected = color(0.9, 0.2, 0.04);
        assert!(equals(&result, &expected));
    }
}
