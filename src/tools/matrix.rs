use std::fmt::Debug;

use super::{
    dual_trait::Algebra,
    transform::{Dimensions, Position, Rotation},
};

/// Matrix is a 2D representation of a vector.

#[derive(Default, Clone, Debug)]
pub struct Matrix<T: Default + Clone> {
    pub values: Vec<T>,
    pub dimensions: Dimensions,
    /// if true, values outside bounds will wrap
    pub wrapping: bool,
}

impl<T: Default + Clone + Sync + Send> Matrix<T> {
    pub fn new(dimensions: Dimensions, wrapping: bool) -> Self {
        Self {
            values: vec![T::default(); dimensions.area()],
            dimensions,
            wrapping,
        }
    }

    fn some_bound(&self, position: Position) -> Option<usize> {
        match (position, self.dimensions, self.wrapping) {
            (Position { x, y }, Dimensions { width, height }, true) => {
                Some((x % width) + ((y % height) * width))
            }
            (Position { x, y }, Dimensions { width, height }, false) if width > x && height > y => {
                Some(x + (y * width))
            }
            _ => None,
        }
    }

    pub fn get(&self, position: Position) -> Option<&T> {
        if let Some(index) = self.some_bound(position) {
            self.values.get(index)
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut T> {
        if let Some(index) = self.some_bound(position) {
            self.values.get_mut(index)
        } else {
            None
        }
    }

    /// will always return value. If out of bounds, it will wrap until in bounds.
    pub fn get_wrap(&self, position: Position) -> &T {
        self.values
            .get(
                (position.x % self.dimensions.width)
                    + ((position.y % self.dimensions.height) * self.dimensions.width),
            )
            .unwrap()
    }

    /// will always return value. If out of bounds, it will wrap until in bounds.
    pub fn get_wrap_mut(&mut self, position: Position) -> &mut T {
        self.values
            .get_mut(
                (position.x % self.dimensions.width)
                    + ((position.y % self.dimensions.height) * self.dimensions.width),
            )
            .unwrap()
    }

    pub fn set(&mut self, position: Position, value: T) {
        if let Some(u) = self.get_mut(position) {
            *u = value;
        }
    }

    /// Applies func to given value and sets it back in the Matrix.
    pub fn apply(&mut self, position: Position, func: impl Fn(&T) -> T) {
        if let Some(u) = self.get_mut(position) {
            *u = func(u);
        }
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (Position, &T)> {
        self.values.iter().enumerate().map(|(i, t)| {
            (
                Position::new(i % self.dimensions.width, i / self.dimensions.width),
                t,
            )
        })
    }

    pub fn enumerate_mut(&mut self) -> impl Iterator<Item = (Position, &mut T)> {
        self.values.iter_mut().enumerate().map(|(i, t)| {
            (
                Position::new(i % self.dimensions.width, i / self.dimensions.width),
                t,
            )
        })
    }

    /// Overlays matrix with other given matrix starting at position (x, y).
    pub fn overlay(&mut self, matrix: &Matrix<T>, position: Position) {
        self.clamp_mut(position, matrix.dimensions)
            .zip(matrix.values.iter())
            .for_each(|(t, u)| *t = u.clone())
    }

    /// Overlays iterator onto matrix starting at position (x, y).
    pub fn overlay_iter<'a>(
        &mut self,
        iter: impl Iterator<Item = &'a T>,
        position: Position,
        dimensions: Dimensions,
    ) where
        T: 'a,
    {
        self.clamp_mut(position, dimensions)
            .zip(iter)
            .for_each(|(t, u)| *t = u.clone())
    }

    /// Overlays matrix only with Some(T) value.
    pub fn transparent_overlay(&mut self, matrix: &Matrix<Option<T>>, position: Position) {
        self.clamp_mut(position, matrix.dimensions)
            .zip(matrix.values.iter())
            .for_each(|(t, u)| {
                if let Some(item) = u {
                    *t = item.clone();
                }
            })
    }

    pub fn transparent_overlay_iter<'a>(
        &mut self,
        iter: impl Iterator<Item = Option<T>>,
        position: Position,
        dimensions: Dimensions,
    ) where
        T: 'a,
    {
        self.clamp_mut(position, dimensions)
            .zip(iter)
            .for_each(|(t, u)| {
                if let Some(item) = u {
                    *t = item.clone()
                }
            })
    }

    /// Lists values in matrix with width and height starting at (x, y). Has possibility to return
    /// less values because they're out of bounds.
    pub fn clamp(&self, position: Position, dimensions: Dimensions) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.dimensions.width)
            .skip(position.y)
            .take(dimensions.height)
            .flat_map(move |chunk| chunk.iter().skip(position.x).take(dimensions.width))
    }

    pub fn clamp_mut(
        &mut self,
        position: Position,
        dimensions: Dimensions,
    ) -> impl Iterator<Item = &mut T> {
        self.values
            .chunks_mut(self.dimensions.width)
            .skip(position.y)
            .take(dimensions.height)
            .flat_map(move |chunk| chunk.iter_mut().skip(position.x).take(dimensions.width))
    }

    pub fn clamp_wrap(
        &self,
        position: Position,
        dimensions: Dimensions,
    ) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.dimensions.width)
            .cycle()
            .skip(position.y)
            .take(dimensions.height)
            .flat_map(move |chunk| chunk.iter().cycle().skip(position.x).take(dimensions.width))
    }

    pub fn clamp_to_matrix(&self, position: Position, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            values: self
                .values
                .chunks(self.dimensions.width)
                .skip(position.y)
                .take(dimensions.height)
                .flat_map(move |chunk| chunk.iter().skip(position.x).take(dimensions.width))
                .cloned()
                .collect::<Vec<_>>(),
            wrapping: false,
        }
    }

    /// enumerates then clamps values
    pub fn enumerate_clamp(
        &self,
        position: Position,
        dimensions: Dimensions,
    ) -> impl Iterator<Item = (Position, &T)> {
        self.clamp(position, dimensions)
            .enumerate()
            .map(move |(i, t)| {
                (
                    position.add(Position::new(
                        i % self.dimensions.width,
                        i / self.dimensions.width,
                    )),
                    t,
                )
            })
    }

    /// enumerates then clamps values
    pub fn enumerate_clamp_wrap(
        &self,
        position: Position,
        dimensions: Dimensions,
    ) -> impl Iterator<Item = (Position, &T)> {
        self.clamp_wrap(position, dimensions)
            .enumerate()
            .map(move |(i, t)| {
                (
                    position.add(Position::new(
                        i % self.dimensions.width,
                        i / self.dimensions.width,
                    )),
                    t,
                )
            })
    }

    /// mirrors the matrix vertically (y = 0)
    pub fn reflect_vertical(&mut self) {
        self.values = self.iter_reflect_vertical().cloned().collect::<Vec<_>>();
    }

    /// returns an iterator of vertically reflected matrix
    pub fn iter_reflect_vertical(&self) -> impl Iterator<Item = &T> {
        self.values.chunks(self.dimensions.width).rev().flatten()
    }

    /// returns a mutible iterator of vertically reflected matrix
    pub fn iter_reflect_vertical_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values
            .chunks_mut(self.dimensions.width)
            .rev()
            .flatten()
    }

    /// mirrors the matrix horizontally (x = 0)
    pub fn reflect_horizontal(&mut self) {
        self.values = self.iter_reflect_horizontal().cloned().collect::<Vec<_>>()
    }

    /// returns an iterator of horizontally reflected matrix
    pub fn iter_reflect_horizontal(&self) -> impl Iterator<Item = &T> {
        self.values
            .chunks(self.dimensions.width)
            .map(|c| c.iter().rev())
            .flatten()
    }

    /// returns a mutible iterator of horizontally reflected matrix
    pub fn iter_reflect_horizontal_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.values
            .chunks_mut(self.dimensions.width)
            .map(|c| c.iter_mut().rev())
            .flatten()
    }

    /// mirrors the matrix on the y = x axis
    pub fn reflect_diagonal(&mut self) {
        self.values = self.iter_reflect_diagonal().cloned().collect::<Vec<_>>();
        self.dimensions.swap()
    }

    /// returns an iterator of diagonally reflected matrix
    pub fn iter_reflect_diagonal(&self) -> impl Iterator<Item = &T> {
        (0..self.dimensions.width).flat_map(move |i| {
            self.values
                .chunks(self.dimensions.width)
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// mirrors the matrix on the y = -x axis
    pub fn reflect_negative_diagonal(&mut self) {
        self.values = self
            .iter_reflect_negative_diagonal()
            .cloned()
            .collect::<Vec<_>>();
        self.dimensions.swap()
    }

    /// returns an iterator of negative diagonally reflected matrix
    pub fn iter_reflect_negative_diagonal(&self) -> impl Iterator<Item = &T> {
        (self.dimensions.width..0).flat_map(move |i| {
            self.values
                .chunks(self.dimensions.width)
                .rev()
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// rotates the matrix to the right
    pub fn rotate_right(&mut self) {
        self.values = self.iter_rotate_right().cloned().collect::<Vec<_>>();
        self.dimensions.swap()
    }

    /// returns an iterator of right rotated matrix
    pub fn iter_rotate_right(&self) -> impl Iterator<Item = &T> {
        (0..self.dimensions.width).flat_map(move |i| {
            self.values
                .chunks(self.dimensions.width)
                .rev()
                .flat_map(move |c| c.iter().skip(i).take(1))
        })
    }

    /// rotates the matrix to the left
    pub fn rotate_left(&mut self) {
        self.values = self.iter_rotate_left().cloned().collect::<Vec<_>>();
        self.dimensions.swap()
    }

    /// returns an iterator of left rotated matrix
    pub fn iter_rotate_left(&self) -> impl Iterator<Item = &T> {
        (0..self.dimensions.width).flat_map(move |i| {
            self.values
                .chunks(self.dimensions.width)
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
    pub fn subdivide_matrix(&self, subdivision_quantities: Dimensions) -> Matrix<Self> {
        let length_dimensions = Dimensions::new(
            self.dimensions.width / subdivision_quantities.width,
            self.dimensions.height / subdivision_quantities.height,
        );
        Matrix {
            values: (0..subdivision_quantities.height)
                .flat_map(|y| {
                    (0..subdivision_quantities.width).map(move |x| {
                        self.clamp_to_matrix(
                            Position::new(
                                x * length_dimensions.width,
                                y * length_dimensions.height,
                            ),
                            length_dimensions,
                        )
                    })
                })
                .collect::<Vec<_>>(),
            dimensions: subdivision_quantities,
            wrapping: self.wrapping,
        }
    }
}
