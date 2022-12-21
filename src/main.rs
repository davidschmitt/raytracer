mod array2d;
mod canvas;
mod color;
mod float;
mod matrix2;
mod matrix3;
mod matrix4;
mod ray;
mod transform;
mod tuple;
mod intersection;
mod shape;

fn main() {
    /*let mut art = canvas::canvas(256, 256);
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
    }*/

    /*
    let mut clock = canvas::canvas(255, 255);
    let point = tuple::point(100.0, 0.0, 0.0);
    let mut point2 = &point;
    let c = color::color(255.0, 255.0, 255.0);
    for i in 0..12 {
        let j = i as f64 * ((2.0 * std::f64::consts::PI) / 12.0);
        point2 = &point;
        let point2 = &matrix4::multiply_tuple(&transform::translation(127.0, 127.0, 0.0), &matrix4::multiply_tuple(&transform::rotation_z(j), &point2));
        let x = point2.x.round();
        let y = point2.y.round();
        canvas::write_pixel(&mut clock, x as usize, y as usize, &c);
    }
    let lines = canvas::to_ppm(&clock);
    for line in lines.iter() {
        println!("{}", line);
    }
    */

    let mut canvas = canvas :: canvas(256, 256);
    let color = color::color(0.0, 0.9, 1.0);
    let sphere = shape::sphere();
    let point = tuple::point(14.0, 19.0, -75.0);
    let scale = transform::scaling(0.04, 0.04, 0.04);

    for x in 0..256 {
        for y in 0..256 {
            let direction = tuple::vector(x as f64 - 128.0, y as f64 - 128.0, 135.0);
            let ray = ray::ray(&point, &direction);
            let ray2 = transform::transform(&ray, &scale);
            let intlist = ray::intersect(&sphere, &ray2);
            let hit = ray::hit(&intlist);
            match hit {
                None => {

                },
                Some (_i) => {
                    canvas::write_pixel(&mut canvas, x, y, &color);
                }
            }
        }
    }

    writeout(canvas)
    
}

pub fn writeout(canvas: canvas::Canvas) {
    let lines = canvas::to_ppm(&canvas);
    for line in lines.iter() {
        println!("{}", line);
    }
} 