use std::fmt::Debug;

use super::transform::Rotation;

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

    fn some_bound(&self, x: usize, y: usize) -> Option<usize> {
        match (self.width, self.height, self.wrapping) {
            (w, h, true) => Some((x % w) + ((y % h) * w)),
            (w, h, false) if w > x && h > y => Some(x + (y * w)),
            _ => None,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        if let Some(index) = self.some_bound(x, y) {
            self.values.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut T> {
        if let Some(index) = self.some_bound(x, y) {
            self.values.get_mut(index)
        } else {
            None
        }
    }

    /// will always return value. If out of bounds, it will wrap until in bounds.
    pub fn get_wrap(&self, x: usize, y: usize) -> &T {
        self.values
            .get((x % self.width) + ((y % self.height) * self.width))
            .unwrap()
    }

    /// will always return value. If out of bounds, it will wrap until in bounds.
    pub fn get_wrap_mut(&mut self, x: usize, y: usize) -> &mut T {
        self.values
            .get_mut((x % self.width) + ((y % self.height) * self.width))
            .unwrap()
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

    /// Overlays iterator onto matrix starting at position (x, y).
    pub fn overlay_iter<'a>(
        &mut self,
        iter: impl Iterator<Item = &'a T>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) where
        T: 'a,
    {
        self.clamp_mut(x, y, width, height)
            .zip(iter)
            .for_each(|(t, u)| *t = u.clone())
    }

    /// Overlays matrix only with Some(T) value.
    pub fn transparent_overlay(&mut self, matrix: &Matrix<Option<T>>, x: usize, y: usize) {
        self.clamp_mut(x, y, matrix.width, matrix.height)
            .zip(matrix.values.iter())
            .for_each(|(t, u)| {
                if let Some(item) = u {
                    *t = item.clone();
                }
            })
    }

    pub fn transparent_overlay_iter<'a>(
        &mut self,
        iter: impl Iterator<Item = &'a Option<T>>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) where
        T: 'a,
    {
        self.clamp_mut(x, y, width, height)
            .zip(iter)
            .for_each(|(t, u)| {
                if let Some(item) = u {
                    *t = item.clone()
                }
            })
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

    pub fn clamp_wrap(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.width)
            .cycle()
            .skip(y)
            .take(height)
            .flat_map(move |chunk| chunk.iter().cycle().skip(x).take(width))
    }

    pub fn clamp_to_matrix(&self, x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            width: 0,
            height: 0,
            values: self
                .values
                .chunks(self.width)
                .skip(y)
                .take(height)
                .flat_map(move |chunk| chunk.iter().skip(x).take(width))
                .cloned()
                .collect::<Vec<_>>(),
            wrapping: false,
        }
    }

    /// enumerates then clamps values
    pub fn enumerate_clamp(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = (usize, usize, &T)> {
        self.clamp(x, y, width, height)
            .enumerate()
            .map(move |(i, t)| (x + (i % self.width), y + (i / self.width), t))
    }

    /// enumerates then clamps values
    pub fn enumerate_clamp_wrap(
        &self,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> impl Iterator<Item = (usize, usize, &T)> {
        self.clamp_wrap(x, y, width, height)
            .enumerate()
            .map(move |(i, t)| (x + (i % self.width), y + (i / self.width), t))
    }

    /// mirrors the matrix vertically (y = 0)
    pub fn reflect_vertical(&mut self) {
        self.values = self.iter_reflect_vertical().cloned().collect::<Vec<_>>();
    }

    /// returns an iterator of vertically reflected matrix
    pub fn iter_reflect_vertical(&self) -> impl Iterator<Item = &T> {
        self.values.chunks(self.width).rev().flatten()
    }

    /// returns a mutible iterator of vertically reflected matrix
    pub fn iter_reflect_vertical_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values.chunks_mut(self.width).rev().flatten()
    }

    /// mirrors the matrix horizontally (x = 0)
    pub fn reflect_horizontal(&mut self) {
        self.values = self.iter_reflect_horizontal().cloned().collect::<Vec<_>>()
    }

    /// returns an iterator of horizontally reflected matrix
    pub fn iter_reflect_horizontal(&self) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.width)
            .map(|c| c.iter().rev())
            .flatten()
    }

    /// returns a mutible iterator of horizontally reflected matrix
    pub fn iter_reflect_horizontal_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values
            .chunks_mut(self.width)
            .map(|c| c.iter_mut().rev())
            .flatten()
    }

    /// mirrors the matrix on the y = x axis
    pub fn reflect_diagonal(&mut self) {
        self.values = self.iter_reflect_diagonal().cloned().collect::<Vec<_>>();
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }

    /// returns an iterator of diagonally reflected matrix
    pub fn iter_reflect_diagonal(&self) -> impl Iterator<Item = &T> {
        (0..self.width).flat_map(move |i| {
            self.values
                .chunks(self.width)
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// mirrors the matrix on the y = -x axis
    pub fn reflect_negative_diagonal(&mut self) {
        self.values = self
            .iter_reflect_negative_diagonal()
            .cloned()
            .collect::<Vec<_>>();
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }

    /// returns an iterator of negative diagonally reflected matrix
    pub fn iter_reflect_negative_diagonal(&self) -> impl Iterator<Item = &T> {
        (self.width..0).flat_map(move |i| {
            self.values
                .chunks(self.width)
                .rev()
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// rotates the matrix to the right
    pub fn rotate_right(&mut self) {
        self.values = self.iter_rotate_right().cloned().collect::<Vec<_>>();
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }

    /// returns an iterator of right rotated matrix
    pub fn iter_rotate_right(&self) -> impl Iterator<Item = &T> {
        (0..self.width).flat_map(move |i| {
            self.values
                .chunks(self.width)
                .rev()
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// rotates the matrix to the left
    pub fn rotate_left(&mut self) {
        self.values = self.iter_rotate_left().cloned().collect::<Vec<_>>();
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }

    /// returns an iterator of left rotated matrix
    pub fn iter_rotate_left(&self) -> impl Iterator<Item = &T> {
        (0..self.width).flat_map(move |i| {
            self.values
                .chunks(self.width)
                .flat_map(move |c| c.iter().rev().skip(i).take(1))
        })
    }

    /// rotates the matrix 180 degrees
    pub fn rotate_180(&mut self) {
        self.values.reverse()
    }

    /// returns an iterator of 180 degree rotated matrix
    pub fn iter_rotate_180(&self) -> impl Iterator<Item = &T> {
        self.values.iter().rev()
    }

    /// rotates the matrix according to rotation direction given
    pub fn rotate(&mut self, rotation: Rotation) {
        match rotation {
            Rotation::DOWN => self.rotate_180(),
            Rotation::LEFT => self.rotate_left(),
            Rotation::RIGHT => self.rotate_right(),
            _ => (),
        }
    }

    /// returns an iterator of matrix rotated according to given rotation direction
    pub fn iter_rotate(&self, rotation: Rotation) -> Box<dyn Iterator<Item = &T> + '_> {
        match rotation {
            Rotation::DOWN => Box::new(self.iter_rotate_180()),
            Rotation::LEFT => Box::new(self.iter_rotate_left()),
            Rotation::RIGHT => Box::new(self.iter_rotate_right()),
            _ => Box::new(self.values.iter()),
        }
    }

    ///returns a matrix that is subdivided into given number of matrices
    pub fn subdivide_matrix(
        &self,
        horizontal_subdivisions: usize,
        vertical_subdivisions: usize,
    ) -> Matrix<Matrix<T>> {
        let horizontal_length = self.width / horizontal_subdivisions;
        let vertical_length = self.height / vertical_subdivisions;
        Matrix::<Matrix<T>> {
            values: (0..vertical_subdivisions)
                .flat_map(|y| {
                    (0..horizontal_subdivisions).map(move |x| {
                        self.clamp_to_matrix(
                            x * horizontal_length,
                            y * vertical_length,
                            horizontal_length,
                            vertical_length,
                        )
                    })
                })
                .collect::<Vec<_>>(),
            width: horizontal_subdivisions,
            height: vertical_subdivisions,
            wrapping: self.wrapping,
        }
    }
}
