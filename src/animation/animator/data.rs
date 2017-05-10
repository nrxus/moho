use super::{Animator, LimitRun};

use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct Data {
    pub max: u32,
    pub duration: Duration,
}

impl Data {
    pub fn new(max: u32, duration: Duration) -> Self {
        Data {
            max: max,
            duration: duration,
        }
    }

    pub fn start(self) -> Animator {
        Animator::new(self)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRun {
        LimitRun::new(self, loops)
    }
}

#[derive(Default, Debug)]
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