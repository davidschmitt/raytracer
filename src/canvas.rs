use crate::array2d;
use crate::color;

pub type Canvas = array2d::Array2D<color::Color>;

pub fn canvas(width: usize, height: usize) -> Canvas {
    return Canvas::new(width, height, color::color(0.0, 0.0, 0.0));
}

pub fn pixel_at(canvas: &Canvas, x: usize, y: usize) -> color::Color {
    return canvas[x][y];
}

pub fn write_pixel(canvas: &mut Canvas, x: usize, y: usize, color: &color::Color) {
    canvas[x][y] = *color;
}

pub fn color_normalize(mut c: f64) -> i32 {
    if c < 0.0 {
        c = 0.0;
    } else if c > 1.0 {
        c = 1.0;
    }
    return (c * 255.0).round() as i32;
}

pub fn to_ppm(canvas: &Canvas) -> Vec<String> {
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("P3"));
    v.push(format!("{} {}", canvas.width(), canvas.height()));
    v.push(String::from("255"));
    for y in 0..canvas.height() {
        let mut line = String::with_capacity(70);
        let mut length = 0;
        for x in 0..canvas.width() {
            if length % 5 == 0 {
                if length != 0 {
                    v.push(line);
                    line = String::with_capacity(70);
                }
            } else {
                line.push_str(" ");
            }
            length += 1;
            let color = pixel_at(&canvas, x, y);
            line.push_str(&format!(
                "{} {} {}",
                color_normalize(color.red),
                color_normalize(color.green),
                color_normalize(color.blue)
            ));
        }
        v.push(line);
    }
    return v;
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
        let pixel = color::color(0.0, 0.0, 0.0);
        assert!(c.width() == width);
        assert!(c.height() == height);
        for w in 0..width {
            for h in 0..height {
                println!("{}, {}", w, h);
                let actual = &pixel_at(&c, w, h);
                if !color::equals(&actual, &pixel) {
                    println!("Wrong: {:?}", actual);
                }
                assert!(color::equals(&actual, &pixel));
            }
        }
    }

    // Page 19
    #[test]
    fn should_permit_writing_pixels() {
        let red = color::color(1.0, 0.0, 0.0);
        let mut c: Canvas = canvas(10, 20);
        write_pixel(&mut c, 2, 3, &red);
        assert!(color::equals(&pixel_at(&c, 2, 3), &red));
    }

    // Page 20
    #[test]
    fn should_generate_valid_ppm_header() {
        let c = canvas(5, 3);
        let lines = to_ppm(&c);
        assert!(lines[0] == "P3");
        assert!(lines[1] == "5 3");
        assert!(lines[2] == "255");
    }

    // Page 21
    #[test]
    fn should_generate_valid_ppm_pixel_data() {
        let mut c = canvas(5, 3);
        let c1 = color::color(1.5, 0.0, 0.0);
        let c2 = color::color(0.0, 0.5, 0.0);
        let c3 = color::color(-0.5, 0.0, 1.0);
        write_pixel(&mut c, 0, 0, &c1);
        write_pixel(&mut c, 2, 1, &c2);
        write_pixel(&mut c, 4, 2, &c3);
        let lines = to_ppm(&c);
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
        let c1 = color::color(1.0, 0.8, 0.6);
        for w in 0..width {
            for h in 0..height {
                write_pixel(&mut c, w, h, &c1);
            }
        }
        let lines = to_ppm(&c);
        assert!(lines[3] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[4] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[5] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
        assert!(lines[6] == "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153");
    }
}
