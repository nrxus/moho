use errors::*;
use input;
use super::State;

use std::time::Duration;

mod fixed;
pub use self::fixed::FixedUpdate;
pub use self::fixed::FixedUpdateState;

type GameState<W, S> = Result<State<Snapshot<W, S>>>;

pub trait World: Sized {
    fn next(self, input: &input::State, elapsed: Duration) -> State<Self>;
}

pub trait Scene<W, S>: Sized {
    fn from(snapshot: Snapshot<W, S>, previous: Self) -> Result<Self>;
}

pub trait Runner<W, S> {
    fn update(&mut self, world: W, elapsed: Duration) -> State<W>;
    fn draw(&mut self, snapshot: &Snapshot<W, S>) -> Result<()>;
    fn elapsed(&mut self) -> Duration;
}

#[derive(Clone, Debug)]
pub struct Snapshot<W, S> {
    pub world: W,
    pub step_state: S,
}

pub trait Step {
    type State;

    fn step<W, R: Runner<W, Self::State>>(
        &self,
        snapshot: Snapshot<W, Self::State>,
        runner: &mut R,
    ) -> GameState<W, Self::State>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    #[derive(Default)]
    pub struct MockRunner<S> {
        time_count: usize,
        pub time_stubs: Vec<Duration>,
        pub quit_on_update: bool,
        pub errors_on_draw: bool,
        pub drawn: Vec<Snapshot<Vec<Duration>, S>>,
    }

    impl<S> Runner<Vec<Duration>, S> for MockRunner<S>
    where
        S: Clone,
    {
        fn update(&mut self, mut world: Vec<Duration>, elapsed: Duration) -> State<Vec<Duration>> {
            if self.quit_on_update {
                State::Quit
            } else {
                world.push(elapsed);
                State::Running(world)
            }
        }

        fn draw(&mut self, snapshot: &Snapshot<Vec<Duration>, S>) -> Result<()> {
            if self.errors_on_draw {
                Err("failed to draw".into())
            } else {
                self.drawn.push(snapshot.clone());
                Ok(())
            }
        }

        fn elapsed(&mut self) -> Duration {
            self.time_count += 1;
            self.time_stubs[self.time_count - 1]
        }
    }

    #[derive(Default, Clone)]
    pub struct MockWorld {
        iterations: Vec<(input::State, Duration)>,
        next_quits: bool,
    }

    impl World for MockWorld {
        fn next(mut self, input: &input::State, elapsed: Duration) -> State<Self> {
            if self.next_quits {
                State::Quit
            } else {
                self.iterations.push((input.clone(), elapsed));
                State::Running(self)
            }
        }
    }

    pub trait GameStateHelper<W, S> {
        fn expect_snapshot(self) -> Snapshot<W, S>;
        fn expect_quit(self);
    }

    impl<W, S> GameStateHelper<W, S> for GameState<W, S> {
        fn expect_snapshot(self) -> Snapshot<W, S> {
            match self.expect("game state in unexpected error state") {
                State::Quit => panic!("game state in unexpected quit state"),
                State::Running(s) => s,
            }
        }

        fn expect_quit(self) {
            if let State::Running(_) = self.expect("game state in unexpected error state") {
                panic!("game state in unexpected running state")
            }
        }
    }
}
