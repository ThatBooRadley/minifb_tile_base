use std::{iter::Cycle, slice::Iter, time::Duration};

use crate::{tile::Tile, tools::timer::Timer};

/// Tile drawn above tilemap
#[derive(Clone)]
pub enum Entity<'a> {
    Animated {
        /// x position of pixels
        x: usize,
        /// y position of pixels
        y: usize,
        /// controls what tile is rendered by entity
        animation_player: &'a AnimationPlayer<'a>,
        /// Image(s) rendered by entity.
        tile: &'a Tile,
        /// order in which entities are drawn if overlapped (lowest first).
        order: usize,
    },
    Simple {
        /// x position of pixels
        x: usize,
        /// y position of pixels
        y: usize,
        /// Image(s) rendered by entity.
        tile: &'a Tile,
        /// order in which entities are drawn if overlapped (lowest first).
        order: usize,
    },
}

impl<'a> Entity<'a> {
    /// creates a new Simple Entity
    pub fn new_simple(x: usize, y: usize, tile: &'a Tile, order: usize) -> Self {
        Self::Simple { x, y, tile, order }
    }

    pub fn new_animated(
        x: usize,
        y: usize,
        animation_player: &'a AnimationPlayer<'a>,
        order: usize,
    ) -> Self {
        Self::Animated {
            x,
            y,
            animation_player,
            tile: animation_player.get_frame(),
            order,
        }
    }

    /// used for overlay
    pub fn to_position_matrix(&self) -> (usize, usize, &Tile) {
        match self {
            Self::Simple { x, y, tile, .. } | Self::Animated { x, y, tile, .. } => (*x, *y, *tile),
        }
    }

    pub fn get_x(&self) -> &usize {
        match self {
            Self::Simple { x, .. } | Self::Animated { x, .. } => x,
        }
    }

    pub fn get_x_mut(&mut self) -> &mut usize {
        match self {
            Self::Simple { x, .. } | Self::Animated { x, .. } => x,
        }
    }

    pub fn get_y(&self) -> &usize {
        match self {
            Self::Simple { y, .. } | Self::Animated { y, .. } => y,
        }
    }

    pub fn get_y_mut(&mut self) -> &mut usize {
        match self {
            Self::Simple { y, .. } | Self::Animated { y, .. } => y,
        }
    }

    pub fn get_tile(&self) -> &Tile {
        match self {
            Self::Simple { tile, .. } | Self::Animated { tile, .. } => tile,
        }
    }

    pub fn get_order(&self) -> &usize {
        match self {
            Self::Simple { order, .. } | Self::Animated { order, .. } => order,
        }
    }
}

/// Works with Entity to allow animations
pub struct AnimationPlayer<'a> {
    /// list of all Tiles to be used in animations
    pub frames: &'a [&'a Tile],
    /// timer to determine when frame should change
    pub timer: Timer,
    /// frame indices with their duration in the order they are to be displayed
    pub reels: &'a mut [AnimationReel<'a>],
    /// current reel index. first number is main index, second is sub-index
    pub index: usize,
}

impl<'a> AnimationPlayer<'a> {
    pub fn new(frames: &'a [&'a Tile], reels: &'a mut [AnimationReel<'a>]) -> Self {
        Self {
            frames,
            timer: Timer::new(Duration::ZERO),
            reels,
            index: 0,
        }
    }

    /// starts reel at given index
    pub fn set_reel(&mut self, index: usize) {
        self.index = index;
        self.timer.reset();
    }

    /// gets current frame (tile)
    pub fn get_frame(&self) -> &Tile {
        self.frames.get(self.get_reel().index).unwrap()
    }

    fn get_reel(&self) -> &AnimationReel {
        self.reels.get(self.index).unwrap()
    }

    fn get_reel_mut(&mut self) -> &mut AnimationReel<'a> {
        self.reels.get_mut(self.index).unwrap()
    }

    /// updates reel timer. if not done, the animation will not change
    pub fn update_timer(&mut self) {
        self.timer.update();
        if self.timer.is_finished() {
            let reel = self.get_reel_mut();
            reel.next();
            self.timer.duration = reel.duration;
        }
    }
}

pub struct AnimationReel<'a> {
    frames: Cycle<Iter<'a, (usize, Duration)>>,
    pub index: usize,
    pub duration: Duration,
}

impl<'a> AnimationReel<'a> {
    pub fn new(frames: &'a [(usize, Duration)]) -> Self {
        let (index, duration) = frames.first().unwrap();
        Self {
            frames: frames.iter().cycle(),
            index: *index,
            duration: *duration,
        }
    }

    pub fn next(&mut self) {
        if let Some((index, duration)) = self.frames.next() {
            self.index = *index;
            self.duration = *duration;
        }
    }
}
