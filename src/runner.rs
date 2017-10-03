use errors::*;
use input;
use timer::{self, Timer};

use std::time::Duration;

pub enum GameState<S> {
    Quit,
    Running(S),
}

impl<S> GameState<S> {
    fn flat_map<F>(self, f: F) -> Self
    where
        F: FnOnce(S) -> Self,
    {
        match self {
            GameState::Quit => GameState::Quit,
            GameState::Running(s) => f(s),
        }
    }
}

pub struct Runner<L: Loop> {
    game_loop: L,
}

impl<L: Loop> Runner<L> {
    pub fn new(game_loop: L) -> Self {
        Runner { game_loop }
    }

    pub fn run<U: Update>(mut self, initial: U) -> Result<()> {
        let mut timer = Timer::new();
        let mut state = initial;

        loop {
            let game_time = timer.update();
            match self.game_loop.advance(state, game_time)? {
                GameState::Quit => return Ok(()),
                GameState::Running(s) => state = s,
            }
        }
    }
}

pub trait Update: Sized {
    fn update(self, input: &input::State, elapsed: Duration) -> GameState<Self>;
    fn tick(self, _: timer::GameTime) -> GameState<Self> {
        GameState::Running(self)
    }

    fn try_into<D>(&self) -> Result<D>
    where
        D: DrawOn<State = Self>,
    {
        D::try_from(self)
    }
}

pub trait DrawOn {
    type State;

    fn draw_on<C>(&self, canvas: &mut C) -> Result<()>;
    fn try_from(state: &Self::State) -> Result<Self>
    where
        Self: Sized;
}

pub trait Loop {
    fn advance<U: Update>(&mut self, state: U, time: timer::GameTime) -> Result<GameState<U>>;
}

pub struct FixedUpdate<E, C> {
    step: Duration,
    max_skip: u32,
    leftover: Duration,
    input_manager: input::Manager<E>,
    canvas: C,
}

impl<E: input::EventPump, C> FixedUpdate<E, C> {
    const NS_IN_SEC: u32 = 1_000_000_000;

    pub fn new(input_manager: input::Manager<E>, canvas: C) -> Self {
        const DEFAULT_RATE: u32 = 60;
        const DEFAULT_MAX_SKIP: u32 = 10;

        FixedUpdate {
            step: Duration::new(0, Self::NS_IN_SEC / DEFAULT_RATE),
            max_skip: DEFAULT_MAX_SKIP,
            leftover: Duration::default(),
            input_manager,
            canvas,
        }
    }

    pub fn rate(mut self, rate: u32) -> Self {
        self.step = Duration::new(0, Self::NS_IN_SEC / rate);
        self
    }

    pub fn max_skip(mut self, max_skip: u32) -> Self {
        self.max_skip = max_skip;
        self
    }
}

impl<E: input::EventPump, C> Loop for FixedUpdate<E, C> {
    fn advance<U>(&mut self, state: U, time: timer::GameTime) -> Result<GameState<U>>
    where
        U: Update,
    {
        self.leftover += time.since_update;
        let mut current = GameState::Running(state);
        let mut loops = 0;
        while self.leftover >= self.step && loops <= self.max_skip {
            match current {
                GameState::Quit => break,
                GameState::Running(s) => {
                    let input = self.input_manager.update();
                    current = if input.game_quit() {
                        GameState::Quit
                    } else {
                        s.update(input, self.step)
                    }
                }
            }
            self.leftover -= self.step;
            loops += 1;
        }

        current = current.flat_map(|s| s.tick(time));

        Ok(current)
    }
}

#[cfg(test)]
mod test {
    use super::*;
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
    fn draws() {}

    #[test]
    fn propagates_draw_failure() {}

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

        assert_eq!(time, state.last_tick_in.unwrap());

        //second advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: Duration::from_millis(400),
        };

        state = subject.advance(state, time).expect_running();

        assert_eq!(time, state.last_tick_in.unwrap());

        //last advance
        state.next_tick_quits = true;
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

        let updates = &state.updates;
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
        assert!(state.updates.is_empty());

        //second advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step / 4,
        };
        state = subject.advance(state, time).expect_running();
        assert!(state.updates.is_empty());

        //third advance
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step / 2,
        };
        state = subject.advance(state, time).expect_running();
        let updates = &state.updates;
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
        let updates = &state.updates;
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
        let updates = &state.updates;
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

        state.next_tick_quits = true;
        subject.advance(state, time).expect_quit();
    }

    #[test]
    fn quits_on_exit_input() {
        let pump = MockEventPump { quit: true };
        let input_manager = input::Manager::new(pump);
        let canvas = MockCanvas {};
        let mut subject = FixedUpdate::new(input_manager, canvas);
        let state = MockState::default();
        let time = timer::GameTime {
            total: Duration::from_secs(1),
            since_update: subject.step,
        };

        subject.advance(state, time).expect_quit();
    }

    fn subject() -> FixedUpdate<MockEventPump, MockCanvas> {
        let pump = MockEventPump::default();
        let input_manager = input::Manager::new(pump);
        let canvas = MockCanvas {};
        FixedUpdate::new(input_manager, canvas)
    }

    #[derive(Default)]
    struct MockState {
        updates: Vec<(input::State, Duration)>,
        last_tick_in: Option<timer::GameTime>,
        next_tick_quits: bool,
        next_update_quits: bool,
    }

    #[derive(Default)]
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

    struct MockCanvas {}

    impl Update for MockState {
        fn update(mut self, input: &input::State, duration: Duration) -> GameState<Self> {
            if self.next_update_quits {
                GameState::Quit
            } else {
                self.updates.push((input.clone(), duration));
                GameState::Running(self)
            }
        }

        fn tick(mut self, time: timer::GameTime) -> GameState<Self> {
            if self.next_tick_quits {
                GameState::Quit
            } else {
                self.last_tick_in = Some(time);
                GameState::Running(self)
            }
        }
    }

    trait GameStateAdvanceHelper {
        type State;
        fn expect_running(self) -> Self::State;
        fn expect_quit(self);
    }

    impl<U> GameStateAdvanceHelper for Result<GameState<U>> {
        type State = U;

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
