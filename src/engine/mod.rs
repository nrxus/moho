pub mod step;

pub use self::step::Step;

use errors::*;
use input;
use renderer::{Canvas, Scene};
use timer::{self, Timer};
use self::step::Runner;

use std::time::Duration;
use std::marker;

#[derive(Debug)]
pub enum State<S> {
    Quit,
    Running(S),
}

impl<S> State<S> {
    pub fn flat_map<F, T>(self, f: F) -> State<T>
    where
        F: FnOnce(S) -> State<T>,
    {
        match self {
            State::Quit => State::Quit,
            State::Running(s) => f(s),
        }
    }

    pub fn map<F, T>(self, f: F) -> State<T>
    where
        F: FnOnce(S) -> T,
    {
        match self {
            State::Quit => State::Quit,
            State::Running(s) => State::Running(f(s)),
        }
    }
}

pub trait World: Sized {
    fn update(self, input: &input::State, elapsed: Duration) -> State<Self>;
    fn tick(self, _: &timer::GameTime) -> Self {
        self
    }
}

pub trait IntoScene<D, S, H> {
    fn try_into(&self, step: &S, helpers: &mut H) -> Result<D>;
}

pub struct App<'a, S, H, R, E>
where
    R: 'a,
    E: 'a,
{
    timer: Timer,
    helpers: H,
    canvas: &'a mut R,
    input_manager: &'a mut input::Manager<E>,
    _marker: marker::PhantomData<*const S>,
}

impl<'a, S, H, R, E> App<'a, S, H, R, E> {
    pub fn new(helpers: H, canvas: &'a mut R, input_manager: &'a mut input::Manager<E>) -> Self {
        App {
            helpers,
            canvas,
            input_manager,
            timer: Timer::new(),
            _marker: marker::PhantomData,
        }
    }
}

impl<'a, 't, W, S, H, C, E, D> Runner<W, S> for App<'a, D, H, C, E>
where
    W: World + IntoScene<D, S, H>,
    C: Canvas<'t, Texture = D::Texture>,
    E: input::EventPump,
    D: Scene,
{
    fn tick(&mut self, world: W, time: &timer::GameTime) -> W {
        world.tick(time)
    }

    fn update(&mut self, world: W, elapsed: Duration) -> State<W> {
        let input = self.input_manager.update();
        if input.game_quit() {
            State::Quit
        } else {
            world.update(input, elapsed)
        }
    }

    fn draw(&mut self, snapshot: &step::Snapshot<W, S>) -> Result<()> {
        let scene = snapshot
            .world
            .try_into(&snapshot.step_state, &mut self.helpers)?;
        self.canvas.clear();
        self.canvas.show(&scene)?;
        self.canvas.present();
        Ok(())
    }

    fn time(&mut self) -> timer::GameTime {
        self.timer.update()
    }
}

pub struct Engine<E, C, S> {
    input_manager: input::Manager<E>,
    canvas: C,
    step: S,
}

impl<'t, E, C, S> Engine<E, C, S>
where
    E: input::EventPump,
    S: Step,
    C: Canvas<'t>,
{
    pub fn new(event_pump: E, canvas: C, step: S) -> Self {
        Engine {
            input_manager: input::Manager::new(event_pump),
            canvas: canvas,
            step: step,
        }
    }

    pub fn run<D, W, H>(&mut self, world: W, helpers: H) -> Result<()>
    where
        W: World + IntoScene<D, S::State, H>,
        D: Scene<Texture = C::Texture>,
    {
        let mut app: App<D, _, _, _> = App::new(helpers, &mut self.canvas, &mut self.input_manager);
        let mut snapshot = step::Snapshot::new::<S>(world);
        loop {
            match self.step.step(snapshot, &mut app)? {
                State::Quit => {
                    break;
                }
                State::Running(s) => snapshot = s,
            }
        }
        Ok(())
    }
}
