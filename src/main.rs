mod float;
mod tuple;
mod color;
mod array2d;
mod canvas;
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
        println!("{}",line);
    }
}