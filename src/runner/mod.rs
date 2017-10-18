pub mod advance;

pub use self::advance::Advance;

use errors::*;
use input;
use renderer::{Canvas, Renderer};
use timer::{self, Timer};

use std::time::Duration;

pub enum GameState<L: Advance, U: Update<L::DrawInfo>> {
    Quit,
    Running(State<L, U>),
}

pub enum UpdateState<W> {
    Quit,
    Running(W),
}

pub struct State<L: Advance, U: Update<L::DrawInfo>> {
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

impl<'t, L: Advance, C: Canvas<'t>> Runner<L, C> {
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
