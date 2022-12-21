use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
static shapeid: AtomicI32 = AtomicI32::new(1);

#[derive(Clone, Copy, Debug)]
pub enum Shape {
    Sphere { id:i32 }
}

pub fn sphere() -> Shape {
    return Shape::Sphere { id: (shapeid.fetch_add(1, Ordering::Relaxed)) }
}

pub fn equals(a: &Shape, b: &Shape) -> bool {
    match (a, b) {
        (Shape::Sphere { id }, Shape::Sphere { .. }) => {
            let aid = id;
            match (b) {
                Shape::Sphere { id } => return aid == id,
            }
        },
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}