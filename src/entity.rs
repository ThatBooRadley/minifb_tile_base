use std::{
    iter::Inspect,
    time::{Duration, Instant, SystemTime},
};

use crate::{
    tile::Tile,
    tools::timer::{self, Timer},
};

/// Tile drawn above tilemap
#[derive(Clone)]
pub struct Entity {
    /// x position of pixels
    pub x: usize,
    /// y position of pixels
    pub y: usize,
    /// Image(s) rendered by entity.
    pub tile: Vec<AnimationFrame>,
    /// Index of current tile
    pub index: usize,
    /// order in which entities are drawn if overlapped (lowest first).
    pub order: usize,
}

impl Entity {
    pub fn new(x: usize, y: usize, tile: Tile, order: usize) -> Self {
        Self {
            x,
            y,
            tile: vec![tile],
            index: 0,
            order,
        }
    }

    /// used for overlay
    pub fn to_position_matrix(&self) -> (usize, usize, &Tile) {
        (self.x, self.y, &self.tile)
    }
}

/// Works with Entity to allow animations
pub struct AnimationPlayer {
    /// list of all Tiles to be used in animations
    pub frames: Vec<Tile>,
    /// timer to determine when frame should change
    pub timer: Timer,
    /// frame indices with their duration in the order they are to be displayed
    pub animations: Vec<Vec<(usize, Duration)>>,
    /// current animation index. first number is main index, second is sub-index
    pub current_index: (usize, usize),
}

impl AnimationPlayer {
    pub fn get_tile(&self) -> Option<&Tile> {
        self.frames.get()
    }
}
