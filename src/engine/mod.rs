pub mod step;

pub use self::step::Step;
use self::step::Runner;
use errors::*;
use input;
use renderer::{Canvas, Scene};
use timer::{self, Timer};

use take_mut;

use std::time::Duration;

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

pub trait NextScene<W, S, H>: Sized {
    fn next(self, snapshot: step::Snapshot<&W, &S>, helpers: &mut H) -> Result<Self>;
}

pub struct App<'a, S, H, R, E>
where
    R: 'a,
    E: 'a,
{
    timer: Timer,
    scene: Result<S>,
    helpers: H,
    canvas: &'a mut R,
    input_manager: &'a mut input::Manager<E>,
}

impl<'a, S, H, R, E> App<'a, S, H, R, E> {
    pub fn new(
        scene: S,
        helpers: H,
        canvas: &'a mut R,
        input_manager: &'a mut input::Manager<E>,
    ) -> Self {
        App {
            helpers,
            canvas,
            input_manager,
            scene: Ok(scene),
            timer: Timer::new(),
        }
    }
}

impl<'a, 't, W, S, H, C, E, D> Runner<W, S> for App<'a, D, H, C, E>
where
    W: World,
    C: Canvas<'t>,
    E: input::EventPump,
    D: Scene<C> + NextScene<W, S, H>,
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

    fn draw(&mut self, snapshot: step::Snapshot<&W, &S>) -> Result<()> {
        {
            let helpers = &mut self.helpers;
            take_mut::take(&mut self.scene, |scene| {
                let scene = scene?;
                scene.next(snapshot, helpers)
            });
        }
        match self.scene {
            Ok(ref scene) => {
                self.canvas.clear();
                self.canvas.show(scene)?;
                self.canvas.present();
                Ok(())
            }
            Err(ref e) => Err(e.description().into()),
        }
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

    pub fn run<D, W, H>(&mut self, world: W, scene: D, helpers: H) -> Result<()>
    where
        W: World,
        D: Scene<C> + NextScene<W, S::State, H>,
    {
        let mut app: App<D, _, _, _> =
            App::new(scene, helpers, &mut self.canvas, &mut self.input_manager);
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
