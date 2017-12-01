pub mod step;

pub use self::step::Step;
use self::step::Runner;
use errors::*;
use input;
use state::State;
use renderer::{Canvas, Scene};
use timer::{self, Timer};

use take_mut;

use std::time::Duration;

pub trait World: Sized {
    type Quit;

    fn update(self, input: &input::State, elapsed: Duration) -> State<Self, Self::Quit>;
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

impl<'a, W, S, H, C, E, D> Runner<W, S> for App<'a, D, H, C, E>
where
    W: World<Quit = ()>,
    C: Canvas,
    E: input::EventPump,
    D: Scene<C> + NextScene<W, S, H>,
{
    fn tick(&mut self, world: W, time: &timer::GameTime) -> W {
        world.tick(time)
    }

    fn update(&mut self, world: W, elapsed: Duration) -> State<W, ()> {
        self.input_manager
            .update()
            .flat_map(|input| world.update(input, elapsed))
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

impl<E, C, S> Engine<E, C, S>
where
    E: input::EventPump,
    S: Step,
    C: Canvas,
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
        W: World<Quit = ()>,
        D: Scene<C> + NextScene<W, S::State, H>,
    {
        let mut app: App<D, _, _, _> =
            App::new(scene, helpers, &mut self.canvas, &mut self.input_manager);
        let mut snapshot = step::Snapshot::new::<S>(world);
        loop {
            match self.step.step(snapshot, &mut app)? {
                State::Quit(_) => {
                    break;
                }
                State::Running(s) => snapshot = s,
            }
        }
        Ok(())
    }
}
