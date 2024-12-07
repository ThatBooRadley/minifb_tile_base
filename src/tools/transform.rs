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
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl Algebra for Size {
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
}

impl Size {
    pub fn area(&self) -> usize {
        self.width * self.height
    }

    pub fn swap(&mut self) {
        let hold = self.width;
        self.width = self.height;
        self.height = hold
    }
}
