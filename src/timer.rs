use std::time::{Duration, Instant};

pub struct Timer {
    last: Instant,
    paused_elapsed: Option<Duration>,
    paused: bool,
    interval: Duration,
}

impl Timer {
    pub fn new(interval: Duration) -> Self {
        Self {
            last: Instant::now(),
            paused_elapsed: None,
            paused: true,
            interval,
        }
    }

    pub fn interval_as_ms(&self) -> u128 {
        self.interval.as_millis()
    }

    pub fn resume(&mut self) {
        if !self.paused {
            return;
        }
        self.last = Instant::now();
        self.paused = false;
    }

    pub fn pause(&mut self) {
        if self.paused {
            return;
        }
        self.paused = true;
        self.paused_elapsed = Some(self.last.elapsed());
    }

    pub fn reset(&mut self) {
        self.last = Instant::now();
        self.pause();
    }

    pub fn reset_time(&mut self) {
        self.last = Instant::now();
    }

    pub fn is_up(&mut self) -> bool {
        if self.paused {
            return false;
        }

        let passed = {
            let mut passed = self.last.elapsed();
            if let Some(from_pause) = self.paused_elapsed {
                passed += from_pause;
            }

            passed
        };
        let is_up = passed >= self.interval;

        if is_up {
            self.paused_elapsed = None;
            true
        } else {
            false
        }
    }
}
