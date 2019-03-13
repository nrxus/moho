use super::{Animator, LimitRun};

use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Data {
    pub max: u32,
    pub duration: Duration,
}

impl Data {
    pub fn start(self) -> Animator {
        Animator::new(self.max, self.duration)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRun {
        LimitRun::new(self.max, self.duration, loops)
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Frame {
    pub index: u32,
    elapsed: Duration,
}

impl Frame {
    pub fn advance(&mut self, delta: Duration, duration: Duration) {
        self.elapsed += delta;
        while self.elapsed >= duration {
            self.index += 1;
            self.elapsed -= duration;
        }
    }
}
