use {timer, Result, State};

use std::time::Duration;

pub mod fixed;
pub use self::fixed::FixedUpdate;

type GameState<W, A, S> = Result<State<Snapshot<W, A, S>, ()>>;

pub trait Runner<W, A, S> {
    fn tick(&mut self, world: W, time: &timer::GameTime) -> W;
    fn update(&mut self, world: W, elapsed: Duration) -> State<W, ()>;
    fn advance(&mut self, assets: A, world: &W, step: &S) -> Result<A>;
    fn time(&mut self) -> timer::GameTime;
}

#[derive(Clone, Debug)]
pub struct Snapshot<W, A, S> {
    pub world: W,
    pub assets: A,
    pub step_state: S,
}

impl<W, A, S: Default> Snapshot<W, A, S> {
    pub fn new<T>(world: W, assets: A) -> Self
    where
        T: Step<State = S>,
    {
        Snapshot {
            world,
            assets,
            step_state: S::default(),
        }
    }
}

impl<W, A, S> Snapshot<W, A, S> {
    pub fn as_ref<'s>(&'s self) -> Snapshot<&'s W, &'s A, &'s S> {
        Snapshot {
            world: &self.world,
            assets: &self.assets,
            step_state: &self.step_state,
        }
    }

    pub fn split<T, F>(self, f: F) -> Snapshot<T, A, S>
    where
        F: FnOnce(W) -> T,
    {
        Snapshot {
            world: f(self.world),
            assets: self.assets,
            step_state: self.step_state,
        }
    }
}

impl<'a, W: Clone, A: Clone, S: Clone> Snapshot<&'a W, &'a A, &'a S> {
    pub fn cloned(&self) -> Snapshot<W, A, S> {
        Snapshot {
            world: W::clone(self.world),
            assets: A::clone(self.assets),
            step_state: S::clone(self.step_state),
        }
    }
}

pub trait Step {
    type State: Default;

    fn step<W, A, R: Runner<W, A, Self::State>>(
        &self,
        snapshot: Snapshot<W, A, Self::State>,
        runner: &mut R,
    ) -> GameState<W, A, Self::State>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct World {
        pub updates: Vec<Duration>,
        pub ticks: Vec<timer::GameTime>,
    }

    #[derive(Default, Clone, Debug, PartialEq)]
    pub struct Assets<S> {
        pub world: World,
        pub step: S,
    }

    #[derive(Default)]
    pub struct MockRunner {
        time_count: usize,
        pub time_stubs: Vec<timer::GameTime>,
        pub quit_on_update: bool,
        pub errors_on_advance: bool,
    }

    impl<S: Clone> Runner<World, Assets<S>, S> for MockRunner {
        fn update(&mut self, mut world: World, elapsed: Duration) -> State<World, ()> {
            if self.quit_on_update {
                State::Quit(())
            } else {
                world.updates.push(elapsed);
                State::Running(world)
            }
        }

        fn advance(&mut self, _: Assets<S>, world: &World, step: &S) -> Result<Assets<S>> {
            if self.errors_on_advance {
                Err("failed to advance assets".into())
            } else {
                Ok(Assets {
                    world: world.clone(),
                    step: step.clone(),
                })
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

    pub trait GameStateHelper<W, A, S> {
        fn expect_snapshot(self) -> Snapshot<W, A, S>;
        fn expect_quit(self);
    }

    impl<W, A, S> GameStateHelper<W, A, S> for GameState<W, A, S> {
        fn expect_snapshot(self) -> Snapshot<W, A, S> {
            match self.expect("game state in unexpected error state") {
                State::Quit(_) => panic!("game state in unexpected quit state"),
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
