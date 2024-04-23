use std::ops::{Index, IndexMut};

#[derive(Copy, Clone, Debug)]
#[repr(C)]
pub struct StaticArray<T: Default + Copy, const SIZE: usize> {
    elements: [T; SIZE],
}

impl<T: Default + Copy, const SIZE: usize> Default for StaticArray<T, SIZE> {
    fn default() -> Self {
        Self {
            elements: [T::default(); SIZE],
        }
    }
}

impl<T: Default + Copy, const SIZE: usize> Index<usize> for StaticArray<T, SIZE> {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a T {
        &self.elements[i]
    }
}

impl<T: Default + Copy, const SIZE: usize> IndexMut<usize> for StaticArray<T, SIZE> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut T {
        &mut self.elements[i]
    }
}
