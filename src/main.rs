use canvas::Canvas;
use color::Color;
use light::Light;
use material::Material;
use ray::Ray;
use shape::Shape;
use transform::Transform;
use tuple::Tuple;
use world::World;

mod array2d;
mod canvas;
mod color;
mod float;
mod intersection;
mod light;
mod material;
mod matrix2;
mod matrix3;
mod matrix4;
mod ray;
mod shape;
mod transform;
mod tuple;
mod world;

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

    let mut canvas = Canvas::new(256, 256);

    let mut s = Shape::sphere();
    s.material = Material::new();
    s.material.color = Color::new(1, 0.2, 1);

    let mut bulb = Light::point(Tuple::point(-25, 20, -30), Color::new(1, 1, 1));

    let origin = Tuple::point(14.0, 19.0, -75.0);
    let scale = Transform::scaling(25.0, 25.0, 25.0);
    s.transform = scale;

    for x in 0..256 {
        for y in 0..256 {
            let direction = Tuple::vector(x as f64 - 128.0, y as f64 - 128.0, 135.0).normalize();
            let r1 = Ray::new(&origin, &direction);
            let intlist = s.intersect(&r1);
            let hit = intlist.hit();
            match hit {
                None => {}
                Some(i) => {
                    let point = r1.position(i.t);
                    let normal = i.s.normal_at(point);
                    let eye = -r1.direction;
                    let c = i.s.material.lighting(bulb.as_ref(), point, eye, normal);
                    canvas[[x, y]] = c;
                }
            }
        }
    }

    canvas.write_out();
}
