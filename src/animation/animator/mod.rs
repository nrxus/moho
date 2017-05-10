mod limit_run;
mod data;

pub use self::limit_run::LimitRun;
pub use self::data::Data;
use self::data::Frame;

use std::time::Duration;

#[derive(Debug)]
pub struct Animator {
    data: Data,
    frame: Frame,
}

impl Animator {
    pub fn new(data: Data) -> Animator {
        Animator {
            data: data,
            frame: Frame::default(),
        }
    }

    pub fn frame(&self) -> u32 {
        self.frame.index
    }

    pub fn num_frames(&self) -> u32 {
        self.data.max
    }

    pub fn animate(&mut self, delta: Duration) -> u32 {
        self.frame.advance(delta, self.data.duration);
        self.frame.index %= self.data.max;
        self.frame.index
    }

    pub fn restart(&mut self) {
        self.frame = Frame::default();
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
