use crate::array2d;
use crate::color::{color, Color, Pixel, ColorMethods};

pub type Canvas = array2d::Array2D<Color>;

pub trait CanvasMethods {
    fn pixel_at(&self, x: usize, y: usize) -> Color;
    fn write_pixel<T: AsRef<Color>>(&mut self, x: usize, y: usize, color: T);
    fn to_ppm(&self) -> Vec<String>;
    fn write_out(&self);
}

impl CanvasMethods for Canvas {
    fn pixel_at(self: &Canvas, x: usize, y: usize) -> Color {
        return self[x][y];
    }

    fn write_pixel<T: AsRef<Color>>(self: &mut Canvas, x: usize, y: usize, color: T) {
        self[x][y] = *color.as_ref();
    }

    fn to_ppm(self: &Canvas) -> Vec<String> {
        let mut v: Vec<String> = Vec::new();
        v.push(String::from("P3"));
        v.push(format!("{} {}", self.width(), self.height()));
        v.push(String::from("255"));
        for y in 0..self.height() {
            let mut line = String::with_capacity(70);
            let mut length = 0;
            for x in 0..self.width() {
                if length % 5 == 0 {
                    if length != 0 {
                        v.push(line);
                        line = String::with_capacity(70);
                    }
                } else {
                    line.push_str(" ");
                }
                length += 1;
                let color = self.pixel_at(x, y);
                let pixel = Pixel::from(&color);
                line.push_str(&format!(
                    "{} {} {}",
                    pixel.red, pixel.green, pixel.blue
                ));
            }
            v.push(line);
        }
        return v;
    }
    fn write_out(&self) {
        let lines = self.to_ppm();
        for line in lines.iter() {
            println!("{}", line);
        }
        
    }
}

pub fn canvas(width: usize, height: usize) -> Canvas {
    return Canvas::new(width, height, color(0.0, 0.0, 0.0));
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 19
    #[test]
    fn should_initialize_correctly() {
        let height = 20;
        let width = 10;
        let c = canvas(width, height);
        let pixel = color(0.0, 0.0, 0.0);
        assert!(c.width() == width);
        assert!(c.height() == height);
        for w in 0..width {
            for h in 0..height {
                println!("{}, {}", w, h);
                let actual = c.pixel_at(w, h);
                if !actual.equals(&pixel) {
                    println!("Wrong: {:?}", actual);
                }
                assert!(actual.equals(&pixel));
            }
        }
    }

    // Page 19
    #[test]
    fn should_permit_writing_pixels() {
        let red = color(1.0, 0.0, 0.0);
        let mut c: Canvas = canvas(10, 20);
        c.write_pixel(2, 3, &red);
        assert!(c.pixel_at(2, 3).equals(&red));
    }

    // Page 20
    #[test]
    fn should_generate_valid_ppm_header() {
        let c = canvas(5, 3);
        let lines = c.to_ppm();
        assert!(lines[0] == "P3");
        assert!(lines[1] == "5 3");
        assert!(lines[2] == "255");
    }

    // Page 21
    #[test]
    fn should_generate_valid_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color(1.5, 0.0, 0.0);
        let c2 = color(0.0, 0.5, 0.0);
        let c3 = color(-0.5, 0.0, 1.0);
        c.write_pixel(0, 0, &c1);
        c.write_pixel(2, 1, &c2);
        c.write_pixel(4, 2, &c3);
        let lines = c.to_ppm();
        println!("{}", lines[3]);
        println!("{}", lines[4]);
        println!("{}", lines[5]);
        assert!(lines[3] == "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0");
        assert!(lines[4] == "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0");
        assert!(lines[5] == "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255");
    }

    // Page 21
    #[test]
    fn should_ensure_short_lines() {
        let width = 10;
        let height = 2;
        let mut c = canvas(width, height);
        let c1 = color(1.0, 0.8, 0.6);
        for w in 0..width {
            for h in 0..height {
                c.write_pixel(w, h, &c1);
            }
        }
        let lines = c.to_ppm();
        assert!(lines[3] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[4] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[5] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[6] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    }
}
