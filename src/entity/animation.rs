use std::{iter::Cycle, slice::Iter, time::Duration};

use crate::{graphics::tile::Tile, tools::timer::Timer};

/// Works with Entity to allow animations
pub struct AnimationPlayer<'a, T: Tile> {
    /// list of all Tiles to be used in animations
    pub frames: &'a [T],
    /// timer to determine when frame should change
    pub timer: Timer,
    /// frame indices with their duration in the order they are to be displayed
    pub reels: &'a mut [AnimationReel<'a>],
    /// current reel index. first number is main index, second is sub-index
    pub index: usize,
}

impl<'a, T: Tile> AnimationPlayer<'a, T> {
    pub fn new(frames: &'a [T], reels: &'a mut [AnimationReel<'a>]) -> Self {
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
    pub fn get_frame(&self) -> &T {
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
