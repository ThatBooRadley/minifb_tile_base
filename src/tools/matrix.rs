use rayon::{iter::*, prelude::*};

#[derive(Default, Clone)]
pub struct Matrix<T: Default + Clone + Sync + Send> {
    pub values: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T: Default + Clone + Sync + Send> Matrix<T> {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            values: vec![T::default(); width * height],
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.values.get(x + (y * self.width))
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        self.values.get_mut(x + (y * self.width))
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if let Some(u) = self.get_mut(x, y) {
            *u = value;
        }
    }

    pub fn apply(&mut self, x: usize, y: usize, func: impl Fn(&T) -> T) {
        if let Some(u) = self.get_mut(x, y) {
            *u = func(u);
        }
    }

    pub fn enumerate(&self) -> impl ParallelIterator<Item = (usize, usize, &T)> {
        self.values
            .par_chunks(self.width)
            .enumerate()
            .flat_map_iter(|(y, chunk)| chunk.iter().enumerate().map(move |(x, t)| (x, y, t)))
    }

    pub fn enumerate_mut(&mut self) -> impl ParallelIterator<Item = (usize, usize, &mut T)> {
        self.values
            .par_chunks_mut(self.width)
            .enumerate()
            .flat_map_iter(|(y, chunk)| chunk.iter_mut().enumerate().map(move |(x, t)| (x, y, t)))
    }
}
