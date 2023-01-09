use crate::color::{Color, Pixel };
use std::ops::{Index, IndexMut};

pub struct Canvas {
    w: usize,
    h: usize,
    pixels: Vec::<Color>,
}

impl Index<[usize; 2]> for Canvas {
    type Output = Color;
    fn index(&self, index: [usize; 2]) -> &Self::Output {
        let idx = self.idx(index[0], index[1]);
        return &self.pixels[idx];
    }
}

impl IndexMut<[usize; 2]> for Canvas {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        let idx = self.idx(index[0], index[1]);
        return self.pixels.index_mut(idx);
    }
}

impl Canvas {

    pub fn width(&self) -> usize {
        return self.w;
    }

    pub fn height(&self) -> usize {
        return self.h;
    }

    pub fn idx(&self, width: usize, height: usize) -> usize {
        return height * self.w + width;
    }

    pub fn new(width: usize, height: usize) -> Canvas {
        let black = Color::new(0, 0, 0);
        return Canvas {
            w: width,
            h: height,
            pixels: vec![black; width * height],
        }
    }

    /*
    pub fn write_pixel<T: AsRef<Color>>(self: &mut Canvas, x: usize, y: usize, color: T) {
        self[x][y] = *color.as_ref();
    }
    */

    pub fn to_ppm(self: &Canvas) -> Vec<String> {
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
                let color = self[[x,y]];
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

    pub fn write_out(&self) {
        let lines = self.to_ppm();
        for line in lines.iter() {
            println!("{}", line);
        }
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Page 19
    #[test]
    fn should_initialize_correctly() {
        let height = 20;
        let width = 10;
        let c = Canvas::new(width, height);
        let pixel = Color::new(0, 0, 0);
        assert!(c.width() == width);
        assert!(c.height() == height);
        for w in 0..width {
            for h in 0..height {
                println!("{}, {}", w, h);
                let actual = c[[w, h]];
                if actual != pixel {
                    println!("Wrong: {:?}", actual);
                }
                assert!(actual == pixel);
            }
        }
    }

    // Page 19
    #[test]
    fn should_permit_writing_pixels() {
        let red = Color::new(1, 0, 0);
        let mut c: Canvas = Canvas::new(10, 20);
        c[[2, 3]] = red;
        assert!(c[[2, 3]] == red);
    }

    // Page 20
    #[test]
    fn should_generate_valid_ppm_header() {
        let c = Canvas::new(5, 3);
        let lines = c.to_ppm();
        assert!(lines[0] == "P3");
        assert!(lines[1] == "5 3");
        assert!(lines[2] == "255");
    }

    // Page 21
    #[test]
    fn should_generate_valid_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c[[0, 0]] = c1;
        c[[2, 1]] = c2;
        c[[4, 2]] = c3;
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
        let mut c = Canvas::new(width, height);
        let c1 = Color::new(1.0, 0.8, 0.6);
        for w in 0..width {
            for h in 0..height {
                c[[w, h]] = c1;
            }
        }
        let lines = c.to_ppm();
        assert!(lines[3] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[4] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[5] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[6] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    }
}
