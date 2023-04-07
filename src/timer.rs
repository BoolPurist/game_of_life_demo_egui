use std::time::{Duration, Instant};

pub struct Timer {
    last: Instant,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            interval,
        }
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
    }

    pub fn is_up(&self) -> bool {
        let passed = self.last.elapsed();
        passed >= self.interval
    }
}
