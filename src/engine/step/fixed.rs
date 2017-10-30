use engine::State;
use super::{GameState, Runner, Snapshot, Step};

use std::time::Duration;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct FixedUpdateState {
    pub leftover: Duration,
    pub interpolation: f64,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct FixedUpdate {
    step: Duration,
    max_skip: u32,
}

impl FixedUpdate {
    const NS_IN_SEC: u32 = 1_000_000_000;

    pub fn rate(mut self, rate: u32) -> Self {
        self.step = Duration::new(0, Self::NS_IN_SEC / rate);
        self
    }

    pub fn max_skip(mut self, max_skip: u32) -> Self {
        self.max_skip = max_skip;
        self
    }
}

impl Default for FixedUpdate {
    fn default() -> Self {
        const DEFAULT_RATE: u32 = 60;
        const DEFAULT_MAX_SKIP: u32 = 10;

        FixedUpdate {
            step: Duration::new(0, Self::NS_IN_SEC / DEFAULT_RATE),
            max_skip: DEFAULT_MAX_SKIP,
        }
    }
}

impl Step for FixedUpdate {
    type State = FixedUpdateState;

    fn step<W, R: Runner<W, FixedUpdateState>>(
        &self,
        snapshot: Snapshot<W, Self::State>,
        runner: &mut R,
    ) -> GameState<W, Self::State> {
        let mut leftover = runner.elapsed() + snapshot.step_state.leftover;
        let mut current = State::Running(snapshot.world);
        let mut loops = 0;

        while leftover >= self.step && loops <= self.max_skip {
            current = current.flat_map(|w| runner.update(w, self.step));
            leftover -= self.step;
            loops += 1;
        }

        let state = current.map(|world| {
            let interpolation =
                f64::from(leftover.subsec_nanos()) / f64::from(self.step.subsec_nanos());

            let step_state = FixedUpdateState {
                leftover,
                interpolation,
            };
            Snapshot { world, step_state }
        });

        if let State::Running(ref s) = state {
            runner.draw(s)?;
        }

        Ok(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use engine::step::mock::*;

    #[test]
    fn default() {
        let subject = FixedUpdate::default();
        assert_eq!(subject.max_skip, 10);
        assert_eq!(subject.step, Duration::new(0, 1_000_000_000 / 60));
    }

    #[test]
    fn custom() {
        let subject = FixedUpdate::default().rate(20).max_skip(4);
        assert_eq!(subject.max_skip, 4);
        assert_eq!(subject.step, Duration::new(0, 50_000_000));
    }

    #[test]
    fn perfect_steps() {
        let subject = FixedUpdate::default();
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step, subject.step];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);
        assert_eq!(snapshot.world[0], subject.step);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 2);
        assert_eq!(snapshot.world[1], subject.step);
    }

    #[test]
    fn fast_updates() {
        let subject = FixedUpdate::default();
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![
            subject.step / 2,
            subject.step / 2,
            subject.step / 2,
            subject.step / 3,
            subject.step / 4,
        ];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 0);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);
        assert_eq!(snapshot.world[0], subject.step);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 2);
        assert_eq!(snapshot.world[1], subject.step);
    }

    #[test]
    fn slow_updates_skips() {
        let subject = FixedUpdate::default().max_skip(10);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step * 3 / 2, subject.step * 3, subject.step * 3 / 2];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);
        assert_eq!(snapshot.world[0], subject.step);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 4);
        assert_eq!(snapshot.world[1], subject.step);
        assert_eq!(snapshot.world[2], subject.step);
        assert_eq!(snapshot.world[3], subject.step);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 6);
        assert_eq!(snapshot.world[4], subject.step);
        assert_eq!(snapshot.world[5], subject.step);
    }

    #[test]
    fn slow_updates_not_enough_skip() {
        let subject = FixedUpdate::default().max_skip(1);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step * 3];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 2);
        assert_eq!(snapshot.world[0], subject.step);
        assert_eq!(snapshot.world[1], subject.step);
    }

    #[test]
    fn slow_updates_no_skip() {
        let subject = FixedUpdate::default().max_skip(0);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step * 2];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.len(), 1);
        assert_eq!(snapshot.world[0], subject.step);
    }

    #[test]
    fn quits() {
        let subject = FixedUpdate::default().max_skip(0);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.quit_on_update = true;
        runner.time_stubs = vec![subject.step];
        subject.step(snapshot, &mut runner).expect_quit();
    }

    #[test]
    fn draws() {
        use std;

        let subject = FixedUpdate::default().max_skip(0);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step / 2, subject.step / 2, subject.step * 5 / 4];

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(runner.drawn.len(), 1);
        assert_eq!(runner.drawn[0].world, vec![]);
        assert!((runner.drawn[0].step_state.interpolation - 0.5).abs() < std::f64::EPSILON);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(runner.drawn.len(), 2);
        assert_eq!(runner.drawn[1].world, vec![subject.step]);
        assert!((runner.drawn[1].step_state.interpolation - 0.0).abs() < std::f64::EPSILON);

        subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(runner.drawn.len(), 3);
        assert_eq!(runner.drawn[2].world, vec![subject.step, subject.step]);
        assert!((runner.drawn[2].step_state.interpolation - 0.25).abs() < 0.0000001);
    }

    #[test]
    fn errors() {
        let subject = FixedUpdate::default().max_skip(0);
        let step_state = FixedUpdateState::default();
        let snapshot = Snapshot {
            world: vec![],
            step_state,
        };
        let mut runner = MockRunner::default();
        runner.time_stubs = vec![subject.step / 2];
        runner.errors_on_draw = true;

        subject.step(snapshot, &mut runner).expect_err("expected error; got a state");
    }
}
