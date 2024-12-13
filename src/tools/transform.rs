use super::dual_trait::Algebra;

#[derive(Clone, Copy, Default)]
pub struct Transform {
    pub position: Position,
    pub rotation: Rotation,
}

#[derive(Clone, Copy, Default)]
pub enum Rotation {
    #[default]
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Algebra for Position {
    type Item = usize;

    fn new(first: Self::Item, last: Self::Item) -> Self {
        Self { x: first, y: last }
    }

    fn first(&self) -> Self::Item {
        self.x
    }

    fn last(&self) -> Self::Item {
        self.y
    }

    fn splat(value: Self::Item) -> Self {
        Self { x: value, y: value }
    }
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Dimensions {
    pub width: usize,
    pub height: usize,
}

impl Algebra for Dimensions {
    type Item = usize;

    fn new(first: Self::Item, last: Self::Item) -> Self {
        Self {
            width: first,
            height: last,
        }
    }

    fn first(&self) -> Self::Item {
        self.width
    }

    fn last(&self) -> Self::Item {
        self.height
    }

    fn splat(value: Self::Item) -> Self {
        Self {
            width: value,
            height: value,
        }
    }
}

impl Dimensions {
    pub fn area(&self) -> usize {
        self.mul_self()
    }

    pub fn swap(&mut self) {
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }
}
