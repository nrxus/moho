use super::{GameState, Runner, Step};
use crate::state::State as AppState;

use std::time::Duration;

pub type Snapshot<W, A> = super::Snapshot<W, A, State>;

#[derive(PartialEq, Default, Debug, Clone)]
pub struct State {
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
    type State = State;

    fn step<W, A>(
        &self,
        snapshot: Snapshot<W, A>,
        runner: &mut impl Runner<W, A, State>,
    ) -> GameState<W, A, State> {
        let time = runner.time();
        let mut current = AppState::Running(runner.tick(snapshot.world, &time));
        let mut leftover = time.since_update + snapshot.step_state.leftover;
        let mut loops = 0;

        while leftover >= self.step && loops <= self.max_skip {
            current = current.flat_map(|w| runner.update(w, self.step));
            leftover -= self.step;
            loops += 1;
        }

        let state = match current {
            AppState::Running(world) => {
                let interpolation =
                    f64::from(leftover.subsec_nanos()) / f64::from(self.step.subsec_nanos());

                let step_state = State {
                    leftover,
                    interpolation,
                };

                let assets = runner.advance(snapshot.assets, &world, &step_state)?;
                AppState::Running(Snapshot {
                    assets,
                    step_state,
                    world,
                })
            }
            _ => AppState::Quit(()),
        };

        Ok(state)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        engine::step::mock::{self, GameStateHelper, MockRunner},
        timer,
    };

    fn game_times(durations: Vec<Duration>) -> Vec<timer::GameTime> {
        let mut total = Duration::from_secs(0);
        durations
            .iter()
            .map(|&d| {
                total += d;
                timer::GameTime {
                    total,
                    since_update: d,
                }
            })
            .collect()
    }

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
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![subject.step, subject.step]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 2);

        assert!(snapshot.world.updates.iter().all(|&u| u == subject.step));
    }

    #[test]
    fn fast_updates() {
        let subject = FixedUpdate::default();
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![
            subject.step / 2,
            subject.step / 2,
            subject.step / 2,
            subject.step / 3,
            subject.step / 4,
        ]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 0);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 2);

        assert!(snapshot.world.updates.iter().all(|&u| u == subject.step));
    }

    #[test]
    fn slow_updates_skips() {
        let subject = FixedUpdate::default().max_skip(10);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![
            subject.step * 3 / 2,
            subject.step * 3,
            subject.step * 3 / 2,
        ]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 4);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 6);

        assert!(snapshot.world.updates.iter().all(|&u| u == subject.step));
    }

    #[test]
    fn slow_updates_not_enough_skip() {
        let subject = FixedUpdate::default().max_skip(1);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![subject.step * 3]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 2);

        assert!(snapshot.world.updates.iter().all(|&u| u == subject.step));
    }

    #[test]
    fn slow_updates_no_skip() {
        let subject = FixedUpdate::default().max_skip(0);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![subject.step * 2]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.updates.len(), 1);

        assert!(snapshot.world.updates.iter().all(|&u| u == subject.step));
    }

    #[test]
    fn quits() {
        let subject = FixedUpdate::default().max_skip(0);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.quit_on_update = true;
        runner.time_stubs = game_times(vec![subject.step]);
        subject.step(snapshot, &mut runner).expect_quit();
    }

    #[test]
    fn advances_assets() {
        use std;

        let subject = FixedUpdate::default().max_skip(0);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![
            subject.step / 2,
            subject.step / 2,
            subject.step * 5 / 4,
        ]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.assets.world, snapshot.world);
        assert!((snapshot.assets.step.interpolation - 0.5).abs() < std::f64::EPSILON);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.assets.world, snapshot.world);
        assert!((snapshot.assets.step.interpolation - 0.0).abs() < std::f64::EPSILON);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.assets.world, snapshot.world);
        assert!((snapshot.assets.step.interpolation - 0.25).abs() < 0.0000001);
    }

    #[test]
    fn errors() {
        let subject = FixedUpdate::default().max_skip(0);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        runner.time_stubs = game_times(vec![subject.step / 2]);
        runner.errors_on_advance = true;

        subject
            .step(snapshot, &mut runner)
            .expect_err("expected error; got a state");
    }

    #[test]
    fn ticks() {
        let subject = FixedUpdate::default().max_skip(0);
        let snapshot = snapshot();
        let mut runner = MockRunner::default();
        let game_times = game_times(vec![subject.step / 2, subject.step, subject.step / 3]);
        runner.time_stubs = game_times.clone();

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.ticks, vec![game_times[0]]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.ticks, vec![game_times[0], game_times[1]]);

        let snapshot = subject.step(snapshot, &mut runner).expect_snapshot();
        assert_eq!(snapshot.world.ticks, game_times);
    }

    fn snapshot() -> Snapshot<mock::World, mock::Assets<State>> {
        Snapshot {
            world: mock::World::default(),
            assets: mock::Assets::default(),
            step_state: State::default(),
        }
    }
}
