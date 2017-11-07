use errors::*;
use timer;
use super::State;

use std::time::Duration;

mod fixed;
pub use self::fixed::FixedUpdate;

type GameState<W, S> = Result<State<Snapshot<W, S>>>;

pub trait Runner<W, S> {
    fn tick(&mut self, world: W, time: &timer::GameTime) -> W;
    fn update(&mut self, world: W, elapsed: Duration) -> State<W>;
    fn draw(&mut self, snapshot: &Snapshot<W, S>) -> Result<()>;
    fn time(&mut self) -> timer::GameTime;
}

#[derive(Clone, Debug)]
pub struct Snapshot<W, S> {
    pub world: W,
    pub step_state: S,
}

impl<W, S: Default> Snapshot<W, S> {
    pub fn new<T>(world: W) -> Self
    where
        T: Step<State = S>,
    {
        Snapshot {
            world,
            step_state: S::default(),
        }
    }
}

pub trait Step {
    type State: Default;

    fn step<W, R: Runner<W, Self::State>>(
        &self,
        snapshot: Snapshot<W, Self::State>,
        runner: &mut R,
    ) -> GameState<W, Self::State>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct World {
        pub updates: Vec<Duration>,
        pub ticks: Vec<timer::GameTime>,
    }

    #[derive(Default)]
    pub struct MockRunner<S> {
        time_count: usize,
        pub time_stubs: Vec<timer::GameTime>,
        pub quit_on_update: bool,
        pub errors_on_draw: bool,
        pub drawn: Vec<Snapshot<World, S>>,
    }

    impl<S> Runner<World, S> for MockRunner<S>
    where
        S: Clone,
    {
        fn update(&mut self, mut world: World, elapsed: Duration) -> State<World> {
            if self.quit_on_update {
                State::Quit
            } else {
                world.updates.push(elapsed);
                State::Running(world)
            }
        }

        fn draw(&mut self, snapshot: &Snapshot<World, S>) -> Result<()> {
            if self.errors_on_draw {
                Err("failed to draw".into())
            } else {
                self.drawn.push(snapshot.clone());
                Ok(())
            }
        }

        fn time(&mut self) -> timer::GameTime {
            self.time_count += 1;
            self.time_stubs[self.time_count - 1]
        }

        fn tick(&mut self, mut world: World, time: &timer::GameTime) -> World {
            world.ticks.push(time.clone());
            world
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
