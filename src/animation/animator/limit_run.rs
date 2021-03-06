use super::data::Frame;

use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct LimitRun {
    max: u32,
    duration: Duration,
    frame: Frame,
    remaining_loops: u32,
}

impl LimitRun {
    pub fn new(max: u32, duration: Duration, loops: u32) -> LimitRun {
        LimitRun {
            max,
            duration,
            remaining_loops: loops,
            frame: Frame::default(),
        }
    }

    pub fn frame(&self) -> Option<u32> {
        if self.remaining_loops > 0 {
            Some(self.frame.index)
        } else {
            None
        }
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    pub fn animate(&mut self, delta: Duration) -> Option<u32> {
        if self.remaining_loops > 0 {
            self.frame.advance(delta, self.duration);
            let elapsed_loops = self.frame.index / self.max;
            if elapsed_loops >= self.remaining_loops {
                self.remaining_loops = 0;
                None
            } else {
                self.frame.index %= self.max;
                self.remaining_loops -= elapsed_loops;
                Some(self.frame.index)
            }
        } else {
            None
        }
    }

    pub fn restart(&mut self, loops: u32) {
        self.remaining_loops = loops;
        self.frame = Frame::default();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::animation::animator::Data;

    #[test]
    fn start() {
        let data = Data {
            max: 2,
            duration: Duration::from_secs(2),
        };
        assert_eq!(data.limit_run_start(2).frame(), Some(0));
    }

    #[test]
    fn start_no_loops() {
        let data = Data {
            max: 2,
            duration: Duration::from_secs(2),
        };
        assert_eq!(data.limit_run_start(0).frame(), None);
    }

    #[test]
    fn stops() {
        let mut animator = LimitRun::new(2, Duration::from_secs(2), 2);

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, Some(1));

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, Some(0));

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, Some(1));

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, None);

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, None);
    }

    #[test]
    fn restart() {
        let mut animator = LimitRun::new(2, Duration::from_secs(2), 1);

        let frame = animator.animate(Duration::from_secs(5));
        assert_eq!(frame, None);

        animator.restart(2);

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, Some(0));

        let frame = animator.animate(Duration::from_secs(6));
        assert_eq!(frame, Some(1));

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, None);
    }
}
