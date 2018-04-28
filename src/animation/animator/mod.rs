mod data;
mod limit_run;

pub use self::data::Data;
use self::data::Frame;
pub use self::limit_run::LimitRun;

use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct Animator {
    max: u32,
    duration: Duration,
    frame: Frame,
}

impl Animator {
    pub fn new(max: u32, duration: Duration) -> Animator {
        Animator {
            max,
            duration,
            frame: Frame::default(),
        }
    }

    pub fn frame(&self) -> u32 {
        self.frame.index
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    pub fn animate(&mut self, delta: Duration) -> u32 {
        self.frame.advance(delta, self.duration);
        self.frame.index %= self.max;
        self.frame.index
    }

    pub fn restart(&mut self) {
        self.frame = Frame::default();
    }

    pub fn limit(self, loops: u32) -> LimitRun {
        LimitRun::new(self.max, self.duration, loops)
    }

    pub fn stop(self) -> Data {
        Data {
            max: self.max,
            duration: self.duration,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start() {
        let animator = Data::new(3, Duration::from_secs(5)).start();
        assert_eq!(animator.frame(), 0);
    }

    #[test]
    fn animate() {
        let mut animator = Data::new(6, Duration::from_secs(5)).start();

        let frame = animator.animate(Duration::from_secs(5));
        assert_eq!(frame, 1);

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, 1);

        let frame = animator.animate(Duration::from_secs(4));
        assert_eq!(frame, 2);

        let frame = animator.animate(Duration::from_secs(4));
        assert_eq!(frame, 3);

        let frame = animator.animate(Duration::from_secs(10));
        assert_eq!(frame, 5);

        assert_eq!(frame, animator.frame());
    }

    #[test]
    fn repeat() {
        let mut animator = Data::new(2, Duration::from_secs(2)).start();

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, 1);

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, 0);

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, 1);

        assert_eq!(frame, animator.frame());
    }

    #[test]
    fn restart() {
        let mut animator = Data::new(2, Duration::from_secs(2)).start();

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, 1);

        animator.restart();
        assert_eq!(animator.frame(), 0);
        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, 0);
        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, 1);
    }
}
