extern crate failure;
extern crate glm;
extern crate moho;
#[macro_use]
extern crate moho_derive;
extern crate sdl2;

use moho::engine::{self, Engine, NextScene};
use moho::{input, timer};
use moho::engine::step::{self, fixed};
use moho::font::{self, Font};
use moho::texture::{self, Image, Texture};
use moho::renderer::{self, align, options, ColorRGBA, Draw, Renderer};
use moho::shape::{Rectangle, Shape};

use std::iter;
use std::rc::Rc;
use std::time::Duration;

type Result<T> = std::result::Result<T, failure::Error>;

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

#[derive(Show)]
struct HoverTextScene<T> {
    image: Image<T>,
}

impl<T: Texture> HoverTextScene<Rc<T>> {
    fn load<F: Font<Texture = T>>(world: &HoverText, font: &F) -> Result<Self> {
        let texture = {
            let color = if world.is_hovering {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            font.texturize(world.text, &color)
        }?;
        let top_left = glm::to_ivec2(world.body.top_left);
        let image = texture.at(renderer::Position::from(top_left));

        Ok(HoverTextScene { image })
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

impl<T: Texture, F: Font<Texture = T>> NextScene<World, fixed::State, Helper<F>> for Scene<Rc<T>> {
    fn next(self, world: &World, _: &fixed::State, helpers: &mut Helper<F>) -> Result<Self> {
        Self::load_dynamic(world, &helpers.font, self.background)
    }
}

struct Scene<T> {
    background: T,
    fps: Image<T>,
    text: HoverTextScene<T>,
}

impl<T: Texture> Scene<Rc<T>> {
    fn load<'t, F, TL>(world: &World, font: &F, loader: &'t TL) -> Result<Self>
    where
        TL: texture::Loader<'t, Texture = T>,
        F: Font<Texture = T>,
    {
        let background = loader.load("examples/background.png")?;
        Self::load_dynamic(world, font, Rc::new(background))
    }

    fn load_dynamic<F>(world: &World, font: &F, background: Rc<T>) -> Result<Self>
    where
        F: Font<Texture = T>,
    {
        let fps = {
            let fps: f64 = world.times.iter().sum();
            let fps = fps / world.times.len() as f64;
            let fps = format!("{:.1}", fps);
            font.texturize(&fps, &ColorRGBA(255, 255, 0, 255))?
                .at(align::top(0).right(1280))
        };
        let text = HoverTextScene::load(&world.text, font)?;
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
    let font_loader = moho::sdl2_helpers::font::Loader::load(&texture_loader).unwrap();

    let helper = Helper::load(&font_loader).unwrap();
    let world = World::load(&helper.font).unwrap();
    let scene = Scene::load(&world, &helper.font, &texture_loader).unwrap();
    let step = step::FixedUpdate::default().rate(30);
    let mut engine = Engine::new(event_pump, canvas, step);
    engine.run(world, scene, helper).unwrap();
}
