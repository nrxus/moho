use errors::*;
use input;
use renderer::{Canvas, Renderer};
use timer::{self, Timer};

use std::time::Duration;

pub enum GameState<L: Loop, U: Update<L::DrawInfo>> {
    Quit,
    Running(State<L, U>),
}

pub enum UpdateState<W> {
    Quit,
    Running(W),
}

pub struct State<L: Loop, U: Update<L::DrawInfo>> {
    pub world: U,
    pub draw: U::Draw,
    pub loop_state: L::State,
}

impl<S> UpdateState<S> {
    pub fn flat_map<F, T>(self, f: F) -> UpdateState<T>
    where
        F: FnOnce(S) -> UpdateState<T>,
    {
        match self {
            UpdateState::Quit => UpdateState::Quit,
            UpdateState::Running(s) => f(s),
        }
    }
}

pub struct Runner<L, C> {
    game_loop: L,
    canvas: C,
}

impl<'t, L: Loop, C: Canvas<'t>> Runner<L, C> {
    pub fn new(game_loop: L, canvas: C) -> Self {
        Runner { game_loop, canvas }
    }

    pub fn run<U>(mut self, initial: State<L, U>) -> Result<()>
    where
        U: Update<L::DrawInfo>,
        U::Draw: Draw<Texture = C::Texture>,
    {
        let mut timer = Timer::new();
        let mut state = initial;

        loop {
            let game_time = timer.update();
            match self.game_loop.advance(state, game_time)? {
                GameState::Quit => return Ok(()),
                GameState::Running(s) => {
                    self.canvas.clear();
                    s.draw.draw(&mut self.canvas)?;
                    self.canvas.present();
                    state = s;
                }
            }
        }
    }
}

pub trait Update<L>: Sized {
    type Draw: Draw;

    fn update(self, input: &input::State, elapsed: Duration) -> UpdateState<Self>;
    fn tick(self, _: timer::GameTime) -> UpdateState<Self> {
        UpdateState::Running(self)
    }
    fn next_draw(&self, previous: Self::Draw, loop_state: L) -> Result<Self::Draw>;
}

pub trait Draw {
    type Texture: ?Sized;

    fn draw<'t, R>(&self, renderer: &mut R) -> Result<()>
    where
        R: Renderer<'t, Texture = Self::Texture>;
}

pub trait Loop: Sized {
    type State: Default;
    type DrawInfo;

    fn advance<U>(
        &mut self,
        state: State<Self, U>,
        time: timer::GameTime,
    ) -> Result<GameState<Self, U>>
    where
        U: Update<Self::DrawInfo>;
}

pub struct FixedUpdate<E> {
    step: Duration,
    max_skip: u32,
    input_manager: input::Manager<E>,
}

impl<E> FixedUpdate<E> {
    const NS_IN_SEC: u32 = 1_000_000_000;

    pub fn new(input_manager: input::Manager<E>) -> Self {
        const DEFAULT_RATE: u32 = 60;
        const DEFAULT_MAX_SKIP: u32 = 10;

        FixedUpdate {
            step: Duration::new(0, Self::NS_IN_SEC / DEFAULT_RATE),
            max_skip: DEFAULT_MAX_SKIP,
            input_manager,
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

impl<E> Loop for FixedUpdate<E>
where
    E: input::EventPump,
{
    type State = Duration;
    type DrawInfo = f64;

    fn advance<U>(
        &mut self,
        state: State<Self, U>,
        time: timer::GameTime,
    ) -> Result<GameState<Self, U>>
    where
        U: Update<f64>,
    {
        let mut leftover = state.loop_state + time.since_update;
        let mut current = UpdateState::Running(state.world);
        let mut loops = 0;
        while leftover >= self.step && loops <= self.max_skip {
            current = current.flat_map(|s| {
                let input = self.input_manager.update();
                if input.game_quit() {
                    UpdateState::Quit
                } else {
                    s.update(input, self.step)
                }
            });
            leftover -= self.step;
            loops += 1;
        }

        current = current.flat_map(|s| s.tick(time));
        match current {
            UpdateState::Quit => Ok(GameState::Quit),
            UpdateState::Running(world) => {
                let interpolation =
                    f64::from(leftover.subsec_nanos()) / f64::from(self.step.subsec_nanos());
                let draw = world.next_draw(state.draw, interpolation)?;
                Ok(GameState::Running(State {
                    world,
                    draw,
                    loop_state: leftover,
                }))
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use sdl2::event::Event;
    use sdl2::rect;
    use renderer::options::{self, Options};
    use renderer::ColorRGBA;

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

    #[derive(Default)]
    struct MockWorld {
        updates: Vec<(input::State, Duration)>,
        last_tick_in: Option<timer::GameTime>,
        next_tick_quits: bool,
        next_update_quits: bool,
        next_try_fails: bool,
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

    #[derive(Default)]
    struct MockCanvas {
        clear_count: usize,
        present_count: usize,
        drawn: Vec<MockDrawState>,
    }

    impl<'t> Canvas<'t> for MockCanvas {
        fn clear(&mut self) {
            self.clear_count += 1;
        }

        fn present(&mut self) {
            self.present_count += 1;
        }
    }

    impl<'t> Renderer<'t> for MockCanvas {
        type Texture = MockDrawState;

        fn set_draw_color(&mut self, _: ColorRGBA) {}

        fn fill_rects(&mut self, _: &[rect::Rect]) -> Result<()> {
            Ok(())
        }

        fn draw_rects(&mut self, _: &[rect::Rect]) -> Result<()> {
            Ok(())
        }

        fn copy(&mut self, texture: &MockDrawState, _: Options) -> Result<()> {
            self.drawn.push(texture.clone());
            Ok(())
        }
    }

    #[derive(Clone, Copy, Default)]
    struct MockDrawState {
        previous_interpolation: f64,
        previous_update_count: usize,
        interpolation: f64,
        update_count: usize,
    }

    impl Draw for MockDrawState {
        type Texture = Self;

        fn draw<'t, R>(&self, renderer: &mut R) -> Result<()>
        where
            R: Renderer<'t, Texture = Self::Texture>,
        {
            renderer.copy(self, options::none())
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
            if self.next_try_fails {
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

    impl<L: Loop, U: Update<L::DrawInfo>> GameStateAdvanceHelper for Result<GameState<L, U>> {
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
