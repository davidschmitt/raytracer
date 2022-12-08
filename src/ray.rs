use crate::matrix4;
use crate::tuple;

#[cfg(test)]
mod tests {
    use super::*;

    /*
    // Page 58
    #[test]
    fn should_create_a_ray() {
        let origin = tuple::point(1.0, 2.0, 3.0);
        let direction = tuple::vector(4.0, 5.0, 6.0);
        let r = ray(&origin, &direction);
        assert!(tuple::equals(&r.origin, origin));
        assert!(tuple::equals(&r.direction, direction));
    }

    #[test]
    fn should_calculate_position() {
        let origin = tuple::point(2.0, 3.0, 4.0);
        let direction = tuple::vector(1.0, 0.0, 0.0);
        let r = ray(&origin, &direction);
        let r1 = position(&r, 0.0);
        assert!(tuple::equals(&r1, &origin));
        let r2 = position(&r, 1.0);
        let e2 = point(3.0, 3.0, 4.0);
        assert!(tuple::equals(&r2, &e2));
        let r3 = position(&r, -1.0);
        let e3 = point(1.0, 3.0, 4.0);
        assert!(tuple::equals(&r3, &e3));
        let r4 = position(&r, 2.5);
        let e4 = point(4.5, 3.0, 4.0);
        assert!(tuple::equals(&r4, &e4));
    }

    #[test]
    fn should_intersect_a_sphere_at_two_points() {
        let r = ray(tuple::point(0.0, 0.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert!(xs.length == 2);
        assert!(float::equals(xs[0], 4.0));
        assert!(float::equals(xs[1], 6.0));
    }

    #[test]
    fn should_intersect_a_sphere_at_tangent_twice() {
        let r = ray(tuple::point(0.0, 1.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert!(xs.length == 2);
        assert!(float::equals(xs[0], 5.0));
        assert!(float::equals(xs[1], 5.0));
    }

    #[test]
    fn should_miss_a_sphere() {
        let r = ray(tuple::point(0.0, 2.0, -5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert!(xs.length == 0);
    }

    #[test]
    fn should_intersect_inside_a_sphere() {
        let r = ray(tuple::point(0.0, 0.0, 0.0), tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert!(xs.length == 2);
        assert!(float::equals(xs[0], -1.0));
        assert!(float::equals(xs[1], 1.0));
    }

    #[test]
    fn should_intersect_behind_a_sphere() {
        let r = ray(tuple::point(0.0, 0.0, 5.0), tuple::vector(0.0, 0.0, 1.0));
        let s = sphere();
        let xs = intersect(&s, &r);
        assert!(xs.length == 2);
        assert!(float::equals(xs[0], -6.0));
        assert!(float::equals(xs[1], -4.0));
    }
    */
}
