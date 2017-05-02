use std::time::Duration;

#[derive(Clone, Copy, Debug, Default)]
pub struct AnimatorData {
    max: u32,
    duration: Duration,
}

impl AnimatorData {
    pub fn new(max: u32, duration: Duration) -> Self {
        AnimatorData {
            max: max,
            duration: duration,
        }
    }

    pub fn start(self) -> Animator {
        Animator::new(self)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRunAnimator {
        LimitRunAnimator::new(self, loops)
    }
}

#[derive(Default, Debug)]
struct Frame {
    index: u32,
    elapsed: Duration,
}

#[derive(Debug)]
pub struct Animator {
    data: AnimatorData,
    frame: Frame,
}

impl Animator {
    pub fn new(data: AnimatorData) -> Animator {
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
        self.frame.elapsed += delta;
        while self.frame.elapsed >= self.data.duration {
            self.frame.index += 1;
            self.frame.elapsed -= self.data.duration;
        }
        self.frame.index = self.frame.index % self.data.max;
        self.frame.index
    }
}

#[derive(Debug)]
pub struct LimitRunAnimator {
    data: AnimatorData,
    frame: Frame,
    remaining_loops: u32,
}

impl LimitRunAnimator {
    pub fn new(data: AnimatorData, loops: u32) -> LimitRunAnimator {
        LimitRunAnimator {
            data: data,
            remaining_loops: loops,
            frame: Frame::default(),
        }
    }

    pub fn frame(&self) -> u32 {
        self.frame.index
    }

    pub fn num_frames(&self) -> u32 {
        self.data.max
    }

    pub fn active(&self) -> bool {
        self.remaining_loops > 0
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start() {
        let animator = AnimatorData::new(3, Duration::from_secs(5)).start();
        assert_eq!(animator.frame(), 0);
    }

    #[test]
    fn animate() {
        let mut animator = AnimatorData::new(6, Duration::from_secs(5)).start();

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
        let mut animator = AnimatorData::new(2, Duration::from_secs(2)).start();

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, 1);

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, 0);

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, 1);

        assert_eq!(frame, animator.frame());
    }

    #[test]
    fn limit_run_start() {
        let mut animator = AnimatorData::new(2, Duration::from_secs(2)).limit_run_start(2);
        assert_eq!(animator.frame(), 0);
        assert!(animator.active());
    }

    #[test]
    fn start_no_loops() {
        let mut animator = AnimatorData::new(2, Duration::from_secs(2)).limit_run_start(0);
        assert!(!animator.active());
    }
}
