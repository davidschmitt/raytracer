// Copied from: https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=3996dc22efee1610ad546f302146d61c

use std::ops::{Index, IndexMut};

pub struct Array2D<T> {
    arr: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T: Clone> Array2D<T> {
    pub fn new(width: usize, height: usize, init: T) -> Self {
        let arr = vec![init; width * height].into_boxed_slice();
        Self { arr, width, height }
    }
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<T> Index<usize> for Array2D<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &[T] {
        let from = index * self.height;
        &self.arr[from..from + self.height]
    }
}
impl<T> IndexMut<usize> for Array2D<T> {
    fn index_mut(&mut self, index: usize) -> &mut [T] {
        let from = index * self.height;
        &mut self.arr[from..from + self.height]
    }
}
