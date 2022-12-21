use crate::shape;

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    pub s: shape::Shape,
}

pub fn intersection(t: f64, s: &shape::Shape) -> Intersection {
    return Intersection { t: t, s: *s };
}

pub fn intersections(list: &[Intersection]) -> Vec<Intersection> {
    let mut vec = Vec::<Intersection>::with_capacity(list.len());
    for i in list {
        vec.push(*i)
    }
    return vec;
}

pub fn equals(a: &Intersection, b: &Intersection) -> bool {
    return (a.t == b.t && shape::equals(&a.s, &b.s));
}

#[cfg(test)]
mod tests {
    use super::*;
}
