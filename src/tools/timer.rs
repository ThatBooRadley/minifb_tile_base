use std::time::{Duration, Instant};

pub struct Timer {
    pub duration: Duration,
    elapsed_time: Duration,
    instant: Instant,
}

impl Timer {
    pub fn new(duration: Duration) -> Self {
        Self {
            duration,
            elapsed_time: Duration::ZERO,
            instant: Instant::now(),
        }
    }

    /// updates elapsed time. must be used to determine if finished
    pub fn update(&mut self) {
        self.elapsed_time = self.instant.elapsed()
    }

    /// returns true if duration is achieved. false if not. (does not update elapsed time)
    pub fn is_finished(&self) -> bool {
        self.elapsed_time >= self.duration
    }

    /// resets elapsed time to 0
    pub fn reset(&mut self) {
        self.elapsed_time = Duration::ZERO;
        self.instant = Instant::now();
    }
}
