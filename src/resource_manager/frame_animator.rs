use std::time::Duration;

#[derive(Default, Clone)]
struct FrameInfo {
    frame: u32,
    elapsed: Duration,
}

impl FrameInfo {
    fn new(frame: u32, elapsed: Duration) -> Self {
        FrameInfo {
            frame: frame,
            elapsed: elapsed,
        }
    }
}

#[derive(Clone)]
pub struct FrameAnimator {
    max: u32,
    duration: Duration,
    repeat: bool,
    current: Option<FrameInfo>,
}

impl FrameAnimator {
    pub fn new(max: u32, duration: Duration, repeat: bool) -> FrameAnimator {
        FrameAnimator {
            max: max,
            duration: duration,
            repeat: repeat,
            current: None,
        }
    }

    pub fn start(&mut self) {
        self.current = Some(Default::default());
    }

    pub fn frame(&self) -> Option<u32> {
        self.current.as_ref().map(|c| c.frame)
    }

    pub fn num_frames(&self) -> u32 {
        self.max
    }

    pub fn animate(&mut self, delta: Duration) {
        self.current = self.current.as_ref().and_then(|f| self.next(f.frame, f.elapsed + delta));
    }

    fn next(&self, current: u32, elapsed: Duration) -> Option<FrameInfo> {
        let mut frame = current;
        let mut remaining = elapsed;
        while remaining >= self.duration {
            frame += 1;
            remaining -= self.duration;
        }
        if frame < self.max || self.repeat {
            Some(FrameInfo::new(frame % self.max, remaining))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_animator() {
        let animator = FrameAnimator::new(3, Duration::from_secs(5), true);
        assert!(animator.frame().is_none());
    }

    #[test]
    fn start() {
        let mut animator = FrameAnimator::new(3, Duration::from_secs(5), true);
        animator.start();
        assert!(animator.frame().is_some());
        assert_eq!(animator.frame().unwrap(), 0);
    }

    #[test]
    fn animate() {
        let mut animator = FrameAnimator::new(6, Duration::from_secs(5), true);
        animator.start();

        animator.animate(Duration::from_secs(5));
        assert_eq!(animator.frame().unwrap(), 1);

        animator.animate(Duration::from_secs(3));
        assert_eq!(animator.frame().unwrap(), 1);

        animator.animate(Duration::from_secs(4));
        assert_eq!(animator.frame().unwrap(), 2);

        animator.animate(Duration::from_secs(4));
        assert_eq!(animator.frame().unwrap(), 3);

        animator.animate(Duration::from_secs(10));
        assert_eq!(animator.frame().unwrap(), 5);
    }

    #[test]
    fn repeat() {
        let mut animator = FrameAnimator::new(2, Duration::from_secs(2), true);
        animator.start();

        animator.animate(Duration::from_secs(2));
        assert_eq!(animator.frame().unwrap(), 1);

        animator.animate(Duration::from_secs(3));
        assert_eq!(animator.frame().unwrap(), 0);

        animator.animate(Duration::from_secs(1));
        assert_eq!(animator.frame().unwrap(), 1);
    }

    #[test]
    fn no_repeat() {
        let mut animator = FrameAnimator::new(2, Duration::from_secs(2), false);
        animator.start();

        animator.animate(Duration::from_secs(2));
        assert_eq!(animator.frame().unwrap(), 1);

        animator.animate(Duration::from_secs(3));
        assert!(animator.frame().is_none());

        animator.start();
        animator.animate(Duration::from_secs(1));
        assert_eq!(animator.frame().unwrap(), 0);

        animator.animate(Duration::from_secs(3));
        assert!(animator.frame().is_none())
    }
}
