use std::time::{Duration, Instant};

pub struct Timer {
    start_time: Instant,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            start_time: Instant::now(),
        }
    }

    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn elapsed(&self) -> Duration {
        Instant::now().duration_since(self.start_time)
    }
}
