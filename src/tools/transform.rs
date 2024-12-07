use std::ops::{Add, AddAssign};

use super::dual_trait::Algebra;

#[derive(Clone, Copy, Default)]
pub struct Transform {
    pub x: usize,
    pub y: usize,
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

pub struct Position {
    pub x: usize,
    pub y: usize,
}
