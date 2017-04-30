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
        Animator::new(self.max, self.duration)
    }
}

#[derive(Default, Debug)]
struct Frame {
    index: u32,
    elapsed: Duration,
}

#[derive(Debug)]
pub struct Animator {
    max: u32,
    duration: Duration,
    frame: Frame,
}

impl Animator {
    pub fn new(max: u32, duration: Duration) -> Animator {
        Animator {
            max: max,
            duration: duration,
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
        self.frame.elapsed += delta;
        while self.frame.elapsed >= self.duration {
            self.frame.index += 1;
            self.frame.elapsed -= self.duration;
        }
        self.frame.index = self.frame.index % self.max;
        self.frame.index
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn start() {
        let data = AnimatorData::new(3, Duration::from_secs(5));
        let animator = data.start();
        assert_eq!(animator.frame(), 0);
    }

    #[test]
    fn animate() {
        let mut animator = Animator::new(6, Duration::from_secs(5));

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
        let mut animator = Animator::new(2, Duration::from_secs(2));

        let frame = animator.animate(Duration::from_secs(2));
        assert_eq!(frame, 1);

        let frame = animator.animate(Duration::from_secs(3));
        assert_eq!(frame, 0);

        let frame = animator.animate(Duration::from_secs(1));
        assert_eq!(frame, 1);

        assert_eq!(frame, animator.frame());
    }
}
