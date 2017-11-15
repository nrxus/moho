extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::engine::{self, Engine, IntoScene};
use moho::errors::*;
use moho::{input, timer};
use moho::engine::step::{self, fixed};
use moho::renderer::font;
use moho::renderer::{self, align, options, ColorRGBA, Font, FontLoader, Renderer, TextureLoader};
use moho::shape::{Rectangle, Shape};

use std::iter;
use std::rc::Rc;
use std::time::Duration;

struct Helper<F, T> {
    font: F,
    background: Rc<T>,
}

impl<F: Font, T> Helper<F, T> {
    fn load<'t, 'f, TL, FL>(texture_loader: &'t TL, font_loader: &'f FL) -> Result<Self>
    where
        TL: TextureLoader<'t, Texture = T>,
        FL: FontLoader<'f, Font = F>,
    {
        let background = texture_loader.load("examples/background.png").map(Rc::new)?;
        let font_details = font::Details {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let font = font_loader.load(&font_details)?;
        Ok(Helper { background, font })
    }
}

struct HoverText {
    text: &'static str,
    body: Rectangle,
    is_hovering: bool,
}

struct HoverTextScene<T> {
    texture: T,
    top_left: glm::IVec2,
}

impl<F, T> IntoScene<HoverTextScene<T>, fixed::State, Helper<F, T>> for HoverText
where
    F: Font<Texture = T>,
{
    fn try_into(&self, _: &fixed::State, helpers: &mut Helper<F, T>) -> Result<HoverTextScene<T>> {
        let texture = {
            let color = if self.is_hovering {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            helpers.font.texturize(self.text, &color)
        }?;
        let top_left = glm::to_ivec2(self.body.top_left);

        Ok(HoverTextScene { texture, top_left })
    }
}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for HoverTextScene<R::Texture> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.copy(
            &self.texture,
            options::at(self.top_left).flip(options::Flip::Horizontal),
        )
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
    fn update(mut self, input: &input::State, _: Duration) -> engine::State<Self> {
        self.text.is_hovering = self.text
            .body
            .contains(&glm::to_dvec2(input.mouse_coords()));
        engine::State::Running(self)
    }

    fn tick(mut self, time: &timer::GameTime) -> Self {
        self.times = iter::once(time.fps())
            .chain(self.times.into_iter())
            .take(10)
            .collect();
        self
    }
}

impl<F, T> IntoScene<Scene<T>, fixed::State, Helper<F, T>> for World
where
    F: Font<Texture = T>,
{
    fn try_into(&self, fixed: &fixed::State, helpers: &mut Helper<F, T>) -> Result<Scene<T>> {
        let background = Rc::clone(&helpers.background);
        let fps = {
            let fps: f64 = self.times.iter().sum();
            let fps = fps / self.times.len() as f64;
            let fps = format!("{:.1}", fps);
            helpers.font.texturize(&fps, &ColorRGBA(255, 255, 0, 255))
        }?;
        let text = self.text.try_into(fixed, helpers)?;

        Ok(Scene {
            background,
            fps,
            text,
        })
    }
}

struct Scene<T> {
    background: Rc<T>,
    fps: T,
    text: HoverTextScene<T>,
}

impl<'t, R: Renderer<'t>> renderer::Scene<R> for Scene<R::Texture> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.copy(&self.background, options::flip(options::Flip::Both))?;
        renderer.copy(&self.background, options::none())?;
        renderer.copy(&self.fps, options::at(align::top(0).right(1280)))?;
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

    let helper = Helper::load(&texture_loader, &font_loader).unwrap();
    let world = World::load(&helper.font).unwrap();
    let step = step::FixedUpdate::default().rate(30);
    let mut engine = Engine::new(event_pump, canvas, step);
    engine
        .run::<Scene<sdl2::render::Texture>, _, _>(world, helper)
        .unwrap();
}
