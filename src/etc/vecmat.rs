#![allow(unused)]
use std::ops::{Index, IndexMut};
use std::fmt::Display;
use std::iter::Enumerate;
use std::slice::Iter;
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
    iter: Enumerate<Iter<'a, T>>,
    mat: &'a VecMat<T>
}

impl<T: Copy> VecMat<T> {
    pub fn new(width: usize, height: usize, default: T) -> Self {
        let data = vec![default; width * height];
        Self { width, height, data }
    }

    pub fn from_data(width: usize, height: usize, data: Vec<T>) -> Self {
        Self { width, height, data }
    }

    pub fn flatten(&self) -> &[T] {
        &self.data
    }

    pub fn indexed_iter(&self) -> VecMaxIndexedIter<T> {
        VecMaxIndexedIter::new(self)
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    ////////////////////////////////////////////////////////////////////////////

    fn index(&self, x: usize, y: usize) -> usize {
        assert!(x < self.width(), "x index out of bounds: {} but width is {}", x, self.width());
        assert!(y < self.height(), "y index out of bounds: {} but height is {}", y, self.height());
        y * self.height + x
    }

    fn coords(&self, index: usize) -> Pos2D {
        (index % self.width, index / self.height)
    }
}

impl<T: Copy, I: PrimInt + Display> Index<(I, I)> for VecMat<T> {
    type Output = T;
    
    fn index(&self, (x, y): (I, I)) -> &Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {}", x)), 
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {}", y))
        );
        &self.data[i]
    }
}

impl<T: Copy, I: PrimInt + Display> IndexMut<(I, I)> for VecMat<T> {
    fn index_mut(&mut self, (x, y): (I, I)) -> &mut Self::Output {
        let i = self.index(
            x.to_usize().unwrap_or_else(|| panic!("X index not valid: {}", x)), 
            y.to_usize().unwrap_or_else(|| panic!("Y index not valid: {}", y))
        );
        &mut self.data[i]
    }
}

impl <'a, T: Copy> VecMaxIndexedIter<'a, T> {
    pub fn new(mat: &'a VecMat<T>) -> Self {
        let iter = mat.data.iter().enumerate();
        Self { mat, iter }
    }
}

impl<'a, T: Copy> Iterator for VecMaxIndexedIter<'a, T> {
    type Item = (Pos2D, T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, x)| (self.mat.coords(i), *x))
    }
}