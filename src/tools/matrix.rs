use std::fmt::Debug;

#[cfg(feature = "parallel")]
use rayon::{iter::*, prelude::*};

/// Matrix is a 2D representation of a vector.

#[derive(Default, Clone, Debug)]
pub struct Matrix<T: Default + Clone + Sync + Send> {
    pub values: Vec<T>,
    pub width: usize,
    pub height: usize,
    /// if true, values outside bounds will wrap
    pub wrapping: bool,
}

impl<T: Default + Clone + Sync + Send> Matrix<T> {
    pub fn new(width: usize, height: usize, wrapping: bool) -> Self {
        Self {
            values: vec![T::default(); width * height],
            width,
            height,
            wrapping,
        }
    }

    fn bound(&self, x: usize, y: usize) -> Option<usize> {
        match (self.width, self.height, self.wrapping) {
            (w, h, true) => Some((x % w) + ((y % h) * w)),
            (w, h, false) if w > x && h > y => Some(x + (y * w)),
            _ => None,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if let Some(index) = self.bound(x, y) {
            self.values.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if let Some(index) = self.bound(x, y) {
            self.values.get_mut(index)
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(u) = self.get_mut(x, y) {
            *u = value;
        }
    }

    /// Applies func to given value and sets it back in the Matrix.
    pub fn apply(&mut self, x: usize, y: usize, func: impl Fn(&T) -> T) {
        if let Some(u) = self.get_mut(x, y) {
            *u = func(u);
        }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.values
            .iter()
            .enumerate()
            .map(|(i, t)| (i % self.width, i / self.width, t))
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (usize, usize, &mut T)> {
        self.values
            .iter_mut()
            .enumerate()
            .map(|(i, t)| (i % self.width, i / self.width, t))
    }

    /// Overlays matrix with other given matrix starting at position (x, y).
    pub fn overlay(&mut self, matrix: &Matrix<T>, x: usize, y: usize) {
        self.clamp_mut(x, y, matrix.width, matrix.height)
            .zip(matrix.values.iter())
            .for_each(|(t, u)| *t = u.clone())
    }

    /// Lists values in matrix with width and height starting at (x, y). Has possibility to return
    /// less values because they're out of bounds.
    pub fn clamp(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.width)
            .skip(y)
            .take(height)
            .flat_map(move |chunk| chunk.iter().skip(x).take(width))
    }

    pub fn clamp_mut(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = &mut T> {
        self.values
            .chunks_mut(self.width)
            .skip(y)
            .take(height)
            .flat_map(move |chunk| chunk.iter_mut().skip(x).take(width))
    }

    #[cfg(feature = "parallel")]
    pub fn par_enumerate(&self) -> impl ParallelIterator<Item = (usize, usize, &T)> {
        self.values
            .par_iter()
            .enumerate()
            .map(|(i, t)| (i % self.width, i / self.width, t))
    }
    #[cfg(feature = "parallel")]
    pub fn par_enumerate_mut(&mut self) -> impl ParallelIterator<Item = (usize, usize, &mut T)> {
        self.values
            .par_iter_mut()
            .enumerate()
            .map(|(i, t)| (i % self.width, i / self.width, t))
    }

    #[cfg(feature = "parallel")]
    pub fn par_overlay(&mut self, matrix: &Matrix<T>, x: usize, y: usize) {
        matrix
            .par_enumerate()
            .for_each(|(sub_x, sub_y, value)| self.set(x + sub_x, y + sub_y, value.clone()))
    }
    #[cfg(feature = "parallel")]
    pub fn par_clamp(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl ParallelIterator<Item = &T> {
        self.values
            .par_chunks(self.width)
            .skip(y)
            .take(height)
            .flat_map_iter(move |chunk| chunk.iter().skip(x).take(width))
    }

    #[cfg(feature = "parallel")]
    pub fn par_clamp_mut(
        &mut self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = &mut T> {
        self.values
            .par_chunks_mut(self.width)
            .skip(y)
            .take(height)
            .flat_map_iter(move |chunk| chunk.iter_mut().skip(x).take(width))
    }
}
