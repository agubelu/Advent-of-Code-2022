// #![allow(unused)]
use std::ops::{Index, IndexMut};
use std::fmt::Display;
use super::utils::Pos2D;
use num_traits::int::PrimInt;

// A 2D-like matrix backed by a Vec
#[derive(Clone)]
pub struct VecMat<T: Copy> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

pub struct VecMaxIndexedIter<'a, T: Copy> {
    mat: &'a VecMat<T>,
    i: usize
}

impl<T: Copy> VecMat<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        Self { width, height, data }
    }

    pub fn get(&self, (x, y): Pos2D) -> T {
        // No bounds check cause yolo
        self.data[self.index(x, y)]
    }

    pub fn set(&mut self, (x, y): Pos2D, elem: T) {
        let i = self.index(x, y);
        self.data[i] = elem;
    }

    pub fn flatten(&self) -> &[T] {
        &self.data
    }

    pub fn indexed_iter(&self) -> VecMaxIndexedIter<T> {
        VecMaxIndexedIter { mat: self, i: 0 }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        y * self.height + x
    }

    pub fn coords(&self, index: usize) -> Pos2D {
        (index % self.width, index / self.height)
    }
}

impl<T: Copy, I: PrimInt + Display> Index<(I, I)> for VecMat<T> {
    type Output = T;
    
    fn index(&self, pos: (I, I)) -> &Self::Output {
        let i = self.index(
            pos.0.to_usize().unwrap_or_else(|| panic!("X index not valid: {}", pos.0)), 
            pos.1.to_usize().unwrap_or_else(|| panic!("Y index not valid: {}", pos.0))
        );
        &self.data[i]
    }
}

impl<T: Copy, I: PrimInt + Display> IndexMut<(I, I)> for VecMat<T> {
    fn index_mut(&mut self, pos: (I, I)) -> &mut Self::Output {
        let i = self.index(
            pos.0.to_usize().unwrap_or_else(|| panic!("X index not valid: {}", pos.0)), 
            pos.1.to_usize().unwrap_or_else(|| panic!("Y index not valid: {}", pos.0))
        );
        &mut self.data[i]
    }
}

impl<'a, T: Copy> Iterator for VecMaxIndexedIter<'a, T> {
    type Item = (Pos2D, T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.mat.size() {
            let item = self.mat.data[self.i];
            let pos = self.mat.coords(self.i);
            self.i += 1;
            Some((pos, item))
        } else {
            None
        }
    }
}