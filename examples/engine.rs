extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::engine::{self, Engine, NextScene};
use moho::errors::*;
use moho::{input, timer};
use moho::engine::step::{self, fixed};
use moho::renderer::font::{self, Font};
use moho::renderer::texture::{self, Image, Texture};
use moho::renderer::{self, align, options, ColorRGBA, Draw, Renderer};
use moho::shape::{Rectangle, Shape};

use std::iter;
use std::time::Duration;

type RefSnapshot<'a, W> = step::Snapshot<&'a W, &'a fixed::State>;

struct Helper<F> {
    font: F,
}

impl<F: Font> Helper<F> {
    fn load<'f, FL>(font_loader: &'f FL) -> Result<Self>
    where
        FL: font::Loader<'f, Font = F>,
    {
        let font_details = font::Details {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        font_loader.load(&font_details).map(|font| Helper { font })
    }
}

struct HoverText {
    text: &'static str,
    body: Rectangle,
    is_hovering: bool,
}

struct HoverTextScene<T> {
    image: Image<T>,
}

impl<T: Texture> HoverTextScene<T> {
    fn load<F: Font<Texture = T>>(snapshot: RefSnapshot<HoverText>, font: &F) -> Result<Self> {
        let texture = {
            let color = if snapshot.world.is_hovering {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            font.texturize(snapshot.world.text, &color)
        }?;
        let top_left = glm::to_ivec2(snapshot.world.body.top_left);
        let image = texture.at(options::Position::from(top_left));

        Ok(HoverTextScene { image })
    }
}

impl<R: Renderer, T: Draw<R>> renderer::Show<R> for HoverTextScene<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.image, options::flip(options::Flip::Horizontal))
    }
}

struct World {
    times: Vec<f64>,
    text: HoverText,
}

impl World {
    fn load<F: Font>(font: &F) -> Result<Self> {
        let text = {
            let text = "HOVER ON ME";
            let dims = font.measure(text)?;
            let body = Rectangle {
                top_left: glm::dvec2(60., 60.),
                dims: glm::to_dvec2(dims),
            };
            HoverText {
                text,
                body,
                is_hovering: false,
            }
        };
        Ok(World {
            text,
            times: vec![],
        })
    }
}

impl engine::World for World {
    type Quit = ();

    fn update(mut self, input: &input::State, _: Duration) -> moho::State<Self, ()> {
        self.text.is_hovering = self.text
            .body
            .contains(&glm::to_dvec2(input.mouse_coords()));
        moho::State::Running(self)
    }

    fn tick(mut self, time: &timer::GameTime) -> Self {
        self.times = iter::once(time.fps())
            .chain(self.times.into_iter())
            .take(10)
            .collect();
        self
    }
}

impl<T: Texture, F: Font<Texture = T>> NextScene<World, fixed::State, Helper<F>> for Scene<T> {
    fn next(self, snapshot: RefSnapshot<World>, helpers: &mut Helper<F>) -> Result<Self> {
        Self::load_snapshot(snapshot, &helpers.font, self.background)
    }
}

struct Scene<T> {
    background: T,
    fps: Image<T>,
    text: HoverTextScene<T>,
}

impl<T: Texture> Scene<T> {
    fn load<'t, F, TL>(world: &World, font: &F, loader: &'t TL) -> Result<Self>
    where
        TL: texture::Loader<'t, Texture = T>,
        F: Font<Texture = T>,
    {
        let background = loader.load("examples/background.png")?;
        let step_state = fixed::State::default();
        let snapshot = RefSnapshot {
            world: world,
            step_state: &step_state,
        };
        Self::load_snapshot(snapshot, font, background)
    }

    fn load_snapshot<F>(snapshot: RefSnapshot<World>, font: &F, background: T) -> Result<Self>
    where
        F: Font<Texture = T>,
    {
        let fps = {
            let fps: f64 = snapshot.world.times.iter().sum();
            let fps = fps / snapshot.world.times.len() as f64;
            let fps = format!("{:.1}", fps);
            font.texturize(&fps, &ColorRGBA(255, 255, 0, 255))?
                .at(align::top(0).right(1280))
        };
        let text = snapshot.split(|w| &w.text);
        let text = HoverTextScene::load(text, font)?;
        Ok(Scene {
            fps,
            text,
            background,
        })
    }
}

impl<R: Renderer, T: Draw<R>> renderer::Show<R> for Scene<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.background, options::flip(options::Flip::Both))?;
        renderer.show(&self.background)?;
        renderer.show(&self.fps)?;
        renderer.show(&self.text)
    }
}

fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let name = "Engine Demo";

    let sdl_ctx = sdl2::init().unwrap();
    let video_ctx = sdl_ctx.video().unwrap();
    let window = video_ctx
        .window(name, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    let event_pump = sdl_ctx.event_pump().unwrap();
    let canvas = window.into_canvas().present_vsync().build().unwrap();
    let texture_loader = canvas.texture_creator();
    let font_loader = moho::renderer::sdl2::font::Loader::load(&texture_loader).unwrap();

    let helper = Helper::load(&font_loader).unwrap();
    let world = World::load(&helper.font).unwrap();
    let scene = Scene::load(&world, &helper.font, &texture_loader).unwrap();
    let step = step::FixedUpdate::default().rate(30);
    let mut engine = Engine::new(event_pump, canvas, step);
    engine.run(world, scene, helper).unwrap();
}
