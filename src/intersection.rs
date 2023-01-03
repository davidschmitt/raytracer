use crate::shape::{Shape, ShapeMethods};

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: f64,
    pub s: Shape,
}

pub trait IntersectionMethods {
    fn equals(&self, peer: &Intersection) -> bool;
}

impl IntersectionMethods for Intersection {
    fn equals(self: &Intersection, peer: &Intersection) -> bool {
        return self.t == peer.t && self.s.equals(&peer.s);
    }
}

pub fn intersection(t: f64, s: &Shape) -> Intersection {
    return Intersection { t: t, s: *s };
}

pub type Intersections = Vec<Intersection>;

pub trait IntersectionsMethods {
    fn hit(&self) -> Option<Intersection>;
}

impl IntersectionsMethods for Intersections {

    fn hit(self: &Vec<Intersection>) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        for i in self {
            if i.t >= 0.0 {
                match closest {
                    None => {
                        closest = Some(*i);
                    }
                    Some(thing) => {
                        if i.t < thing.t {
                            closest = Some(*i)
                        }
                    }
                }
            }
        }
        return closest;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
