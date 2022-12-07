mod array2d;
mod canvas;
mod color;
mod float;
mod matrix2;
mod matrix3;
mod matrix4;
mod tuple;
fn main() {
    let mut art = canvas::canvas(256, 256);
    for x in 0..art.width() {
        for y in 0..art.height() {
            let red = x as f64 / 255.0;
            let green = y as f64 / 255.0;
            let blue = (255 - x) as f64 / 255.0;
            let c = color::color(red, green, blue);
            canvas::write_pixel(&mut art, x, y, &c);
        }
    }
    let lines = canvas::to_ppm(&art);
    for line in lines.iter() {
        println!("{}", line);
    }
}
