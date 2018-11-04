pub mod step;

pub use self::step::Step;

use self::step::Runner;
use crate::{
    input,
    renderer::{Canvas, Show},
    timer::{self, Timer},
    Result, State,
};

use std::time::Duration;

pub trait World: Sized {
    type Quit;

    fn update(self, input: &input::State, elapsed: Duration) -> State<Self, Self::Quit>;
    fn tick(self, _: &timer::GameTime) -> Self {
        self
    }
}

pub trait NextScene<W, S, H>: Sized {
    fn next(self, world: &W, step: &S, helpers: &mut H) -> Result<Self>;
}

pub struct App<'a, E, H> {
    input_manager: &'a mut input::Manager<E>,
    helpers: H,
    timer: Timer,
}

impl<'a, E, H> App<'a, E, H> {
    pub fn new(helpers: H, input_manager: &'a mut input::Manager<E>) -> Self {
        App {
            input_manager,
            helpers,
            timer: Timer::new(),
        }
    }
}

impl<W, A, S, E, H> Runner<W, A, S> for App<'_, E, H>
where
    W: World<Quit = ()>,
    A: NextScene<W, S, H>,
    E: input::EventPump,
{
    fn tick(&mut self, world: W, time: &timer::GameTime) -> W {
        world.tick(time)
    }

    fn update(&mut self, world: W, elapsed: Duration) -> State<W, ()> {
        self.input_manager
            .update()
            .flat_map(|input| world.update(input, elapsed))
    }

    fn advance(&mut self, assets: A, world: &W, step: &S) -> Result<A> {
        assets.next(world, step, &mut self.helpers)
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
            canvas,
            step,
        }
    }

    pub fn run<W, H>(
        &mut self,
        world: W,
        assets: impl Show<C> + NextScene<W, S::State, H>,
        helpers: H,
    ) -> Result<()>
    where
        W: World<Quit = ()>,
    {
        let mut app = App::new(helpers, &mut self.input_manager);
        let mut snapshot = step::Snapshot::new::<S>(world, assets);
        loop {
            match self.step.step(snapshot, &mut app)? {
                State::Quit(_) => {
                    break;
                }
                State::Running(s) => {
                    self.canvas.clear();
                    self.canvas.show(&s.assets)?;
                    self.canvas.present();
                    snapshot = s;
                }
            }
        }
        Ok(())
    }
}
