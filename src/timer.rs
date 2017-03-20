use std::time::{Duration, Instant};

#[derive(Debug,Clone)]
pub struct GameTime {
    pub total: Duration,
    pub since_update: Duration,
}

impl GameTime {
    pub fn fps(&self) -> f64 {
        const NANO_IN_SEC: f64 = 1000000000.;
        let duration = self.since_update.as_secs() as f64 +
                       self.since_update.subsec_nanos() as f64 / NANO_IN_SEC;
        1. / duration
    }
}

pub struct Timer {
    start: Instant,
    last_update: Instant,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        let now = Instant::now();
        Timer {
            start: now,
            last_update: now,
        }
    }

    pub fn update(&mut self) -> GameTime {
        let now = Instant::now();
        let since_update = now - self.last_update;
        self.last_update = now;
        GameTime {
            total: now - self.start,
            since_update: since_update,
        }
    }
}
