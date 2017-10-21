use errors::*;
use runner::{Draw, GameState, State, Update, UpdateState};
use timer;

use std::time::Duration;

pub trait Advance: Sized {
    type State: Default;
    type DrawInfo;

    fn advance<W, U, D, T: Draw>(
        &self,
        state: State<Self, W>,
        time: timer::GameTime,
        update: U,
        draw: D,
    ) -> Result<GameState<Self, W>>
    where
        W: Update<Self::DrawInfo, Draw = T>,
        U: FnMut(W, Duration) -> UpdateState<W>,
        D: FnOnce(&T) -> Result<()>;
}

pub struct FixedUpdate {
    step: Duration,
    max_skip: u32,
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

impl Advance for FixedUpdate {
    type State = Duration;
    type DrawInfo = f64;

    fn advance<W, U, D, T: Draw>(
        &self,
        state: State<Self, W>,
        time: timer::GameTime,
        mut update: U,
        draw: D,
    ) -> Result<GameState<Self, W>>
    where
        W: Update<Self::DrawInfo, Draw = T>,
        U: FnMut(W, Duration) -> UpdateState<W>,
        D: FnOnce(&T) -> Result<()>,
    {
        let mut leftover = state.loop_state + time.since_update;
        let mut current = UpdateState::Running(state.world);
        let mut loops = 0;
        while leftover >= self.step && loops <= self.max_skip {
            current = current.flat_map(|s| update(s, self.step));
            leftover -= self.step;
            loops += 1;
        }

        current = current.flat_map(|s| s.tick(time));
        match current {
            UpdateState::Quit => Ok(GameState::Quit),
            UpdateState::Running(world) => {
                let interpolation =
                    f64::from(leftover.subsec_nanos()) / f64::from(self.step.subsec_nanos());
                let draw_state = world.next_draw(state.draw, interpolation)?;
                draw(&draw_state)?;
                Ok(GameState::Running(State {
                    world,
                    draw: draw_state,
                    loop_state: leftover,
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use runner::Draw;
    use renderer::Renderer;
    use sdl2::event::Event;

    #[test]
    fn default() {
        let subject = subject();
        assert_eq!(subject.max_skip, 10);
        assert_eq!(subject.step, Duration::new(0, 1_000_000_000 / 60));
    }

    #[test]
    fn custom() {
        let subject = subject().rate(20).max_skip(4);
        assert_eq!(subject.max_skip, 4);
        assert_eq!(subject.step, Duration::new(0, 50_000_000));
    }

    #[test]
    fn draws() {
        use std;

        let mut subject = subject();
        let mut state = MockState::default();

        //draw no update
        let time = timer::GameTime {
            total: Duration::default(),
            since_update: subject.step / 2,
        };
        state = subject.advance(state, time).expect_running();
        assert!((state.draw.interpolation - 0.5) < std::f64::EPSILON);
        assert_eq!(state.draw.update_count, 0);

        //draw with update
        let time = timer::GameTime {
            total: Duration::default(),
            since_update: subject.step * 3 / 4,
        };
        state = subject.advance(state, time).expect_running();
        assert!((state.draw.interpolation - 0.25) < std::f64::EPSILON);
        assert_eq!(state.draw.update_count, 1);
        assert!((state.draw.previous_interpolation - 0.5) < std::f64::EPSILON);
        assert_eq!(state.draw.previous_update_count, 0);

        //draw with multiple updates
        let time = timer::GameTime {
            total: Duration::default(),
            since_update: subject.step * 3,
        };
        state = subject.advance(state, time).expect_running();
        assert!((state.draw.interpolation - 0.25) < std::f64::EPSILON);
        assert_eq!(state.draw.update_count, 4);
        assert!((state.draw.previous_interpolation - 0.5) < std::f64::EPSILON);
        assert_eq!(state.draw.previous_update_count, 1);
    }

    #[test]
    fn propagates_draw_failure() {
        let mut subject = subject();
        let mut state = MockState::default();
        state.world.next_draw_fails = true;

        //draw no update
        let time = timer::GameTime {
            total: Duration::default(),
            since_update: subject.step / 2,
        };

        assert!(subject.advance(state, time).is_err());
    }

    #[test]
    fn ticks() {
        let mut subject = subject();
        let mut state = MockState::default();

        //first advance
        let time = timer::GameTime {
            total: Duration::default(),
            since_update: Duration::default(),
        };

        state = subject.advance(state, time).expect_running();

        assert_eq!(time, state.world.last_tick_in.unwrap());

        //second advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: Duration::from_millis(400),
        };

        state = subject.advance(state, time).expect_running();

        assert_eq!(time, state.world.last_tick_in.unwrap());

        //last advance
        state.world.next_tick_quits = true;
        subject.advance(state, time).expect_quit()
    }

    #[test]
    fn perfect_step_update() {
        let mut subject = subject();
        let mut state = MockState::default();

        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step,
        };

        state = subject.advance(state, time).expect_running();

        let updates = &state.world.updates;
        assert_eq!(updates.len(), 1);
        let (ref input, ref duration) = updates[0];
        assert_eq!(*input, input::State::default());
        assert_eq!(*duration, subject.step);
    }

    #[test]
    fn fast_updates() {
        let mut subject = subject();
        let mut state = MockState::default();

        //first advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step / 2,
        };
        state = subject.advance(state, time).expect_running();
        assert!(state.world.updates.is_empty());

        //second advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step / 4,
        };
        state = subject.advance(state, time).expect_running();
        assert!(state.world.updates.is_empty());

        //third advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step / 2,
        };
        state = subject.advance(state, time).expect_running();
        let updates = &state.world.updates;
        assert_eq!(updates.len(), 1);
        let (ref input, ref duration) = updates[0];
        assert_eq!(*input, input::State::default());
        assert_eq!(*duration, subject.step);
    }

    #[test]
    fn slow_updates_skips() {
        let mut subject = subject().max_skip(2);
        let mut state = MockState::default();
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step * 4,
        };
        state = subject.advance(state, time).expect_running();
        let updates = &state.world.updates;
        assert_eq!(updates.len(), 3);
        updates.iter().for_each(|&(ref i, ref d)| {
            assert_eq!(*i, input::State::default());
            assert_eq!(*d, subject.step);
        });
    }

    #[test]
    fn slow_updates_no_skip() {
        let mut subject = subject().max_skip(0);
        let mut state = MockState::default();
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step * 2,
        };
        state = subject.advance(state, time).expect_running();
        let updates = &state.world.updates;
        assert_eq!(updates.len(), 1);
        let (ref input, ref duration) = updates[0];
        assert_eq!(*input, input::State::default());
        assert_eq!(*duration, subject.step);
    }

    #[test]
    fn quits_on_update() {
        let mut subject = subject();
        let mut state = MockState::default();
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step,
        };

        state.world.next_tick_quits = true;
        subject.advance(state, time).expect_quit();
    }

    #[test]
    fn quits_on_exit_input() {
        let pump = MockEventPump { quit: true };
        let input_manager = input::Manager::new(pump);
        let mut subject = FixedUpdate::new(input_manager);
        let state = MockState::default();
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step,
        };

        subject.advance(state, time).expect_quit();
    }

    fn subject() -> FixedUpdate<MockEventPump> {
        let pump = MockEventPump::default();
        let input_manager = input::Manager::new(pump);
        FixedUpdate::new(input_manager)
    }

    #[derive(Default, Debug)]
    struct MockWorld {
        updates: Vec<(input::State, Duration)>,
        last_tick_in: Option<timer::GameTime>,
        next_tick_quits: bool,
        next_update_quits: bool,
        next_draw_fails: bool,
    }

    #[derive(Default, Debug)]
    struct MockEventPump {
        quit: bool,
    }

    impl input::EventPump for MockEventPump {
        fn poll_event(&mut self) -> Option<Event> {
            if self.quit {
                self.quit = false;
                Some(Event::Quit { timestamp: 50 })
            } else {
                None
            }
        }
    }

    #[derive(Clone, Copy, Default, Debug)]
    struct MockDrawState {
        previous_interpolation: f64,
        previous_update_count: usize,
        interpolation: f64,
        update_count: usize,
    }

    impl Draw for MockDrawState {
        type Texture = ();

        fn draw<'t, R: Renderer<'t, Texture = ()>>(&self, _: &mut R) -> Result<()> {
            Ok(())
        }
    }

    impl Update<f64> for MockWorld {
        type Draw = MockDrawState;

        fn update(mut self, input: &input::State, duration: Duration) -> UpdateState<Self> {
            if self.next_update_quits {
                UpdateState::Quit
            } else {
                self.updates.push((input.clone(), duration));
                UpdateState::Running(self)
            }
        }

        fn tick(mut self, time: timer::GameTime) -> UpdateState<Self> {
            if self.next_tick_quits {
                UpdateState::Quit
            } else {
                self.last_tick_in = Some(time);
                UpdateState::Running(self)
            }
        }

        fn next_draw(&self, previous: MockDrawState, interpolation: f64) -> Result<MockDrawState> {
            if self.next_draw_fails {
                Err("failed to convert".into())
            } else {
                Ok(MockDrawState {
                    interpolation,
                    update_count: self.updates.len(),
                    previous_interpolation: previous.interpolation,
                    previous_update_count: previous.update_count,
                })
            }
        }
    }

    type MockState = State<FixedUpdate<MockEventPump>, MockWorld>;

    impl Default for MockState {
        fn default() -> MockState {
            State {
                world: MockWorld::default(),
                draw: MockDrawState::default(),
                loop_state: Duration::default(),
            }
        }
    }

    trait GameStateAdvanceHelper {
        type State;
        fn expect_running(self) -> Self::State;
        fn expect_quit(self);
    }

    impl<L: Advance, U: Update<L::DrawInfo>> GameStateAdvanceHelper for Result<GameState<L, U>> {
        type State = State<L, U>;

        fn expect_running(self) -> Self::State {
            match self.expect("got an error as a game state") {
                GameState::Quit => panic!("advancing caused an unexpected game quit"),
                GameState::Running(s) => s,
            }
        }

        fn expect_quit(self) {
            if let GameState::Quit = self.expect("got an error as a game state") {
            } else {
                panic!("advancing the loop should have quit")
            }
        }
    }
}
