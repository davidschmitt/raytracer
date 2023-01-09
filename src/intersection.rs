use crate::float::Float;
use crate::shape::Shape;
use std::ops::Index;

#[derive(Clone, Copy, Debug)]
pub struct Intersection {
    pub t: Float,
    pub s: Shape,
}

impl PartialEq<Intersection> for Intersection {
    fn eq(&self, rhs: &Intersection) -> bool {
        return self.t == rhs.t && self.s == rhs.s;
    }
}

impl AsRef<Intersection> for Intersection {
    fn as_ref(&self) -> &Intersection {
        return self;
    }
}

impl Intersection {
    pub fn new<T: Into<f64>, S: AsRef<Shape>>(t: T, s: S) -> Intersection {
        return Intersection {
            t: Float::from(t.into()),
            s: *(s.as_ref()),
        };
    }
}

pub struct Intersections(Vec<Intersection>);

impl Index<usize> for Intersections {
    type Output = Intersection;
    fn index(&self, index: usize) -> &Self::Output {
        return &self.0[index];
    }
}

impl Intersections {
    pub fn new() -> Intersections {
        return Intersections(Vec::new());
    }

    pub fn from(values: &[Intersection]) -> Intersections {
        let mut list = Intersections::new();
        for value in values {
            list.push(value);
        }
        return list;
    }

    pub fn hit(self: &Intersections) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;
        for i in &self.0 {
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

    pub fn push<S: AsRef<Intersection>>(&mut self, i: S) {
        self.0.push(*(i.as_ref()));
    }

    pub fn len(&self) -> usize {
        return self.0.len();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
