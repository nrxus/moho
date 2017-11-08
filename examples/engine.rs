extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::engine::{self, Engine, IntoScene};
use moho::errors::*;
use moho::{input, timer};
use moho::engine::step::{self, fixed};
use moho::renderer::{self, align, options, ColorRGBA, Font, FontDetails, FontLoader,
                     FontTexturizer, Renderer, TextureLoader};
use moho::shape::{Rectangle, Shape};

use std::rc::Rc;
use std::time::Duration;

struct Helper<'t, F, TL>
where
    TL: 't + FontTexturizer<'t, F>,
{
    font: F,
    texture_loader: &'t TL,
    background: Rc<TL::Texture>,
}

impl<'t, F, TL> Helper<'t, F, TL>
where
    TL: FontTexturizer<'t, F> + TextureLoader<'t, Texture = <TL as FontTexturizer<'t, F>>::Texture>,
{
    fn load<'f, FL>(texture_loader: &'t TL, font_loader: &'f FL) -> Result<Self>
    where
        FL: FontLoader<'f, Font = F>,
    {
        let background = texture_loader.load("examples/background.png").map(Rc::new)?;
        let font_details = FontDetails {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let font = font_loader.load(&font_details)?;
        Ok(Helper {
            background,
            font,
            texture_loader,
        })
    }
}

struct World {
    fps: f64,
    button: Rectangle,
    text: &'static str,
    cursor: glm::IVec2,
}

impl World {
    fn load<F: Font>(font: &F) -> Result<Self> {
        let text = "HOVER ON ME";
        let button_dims = font.measure(text)?;
        let button = Rectangle {
            top_left: glm::dvec2(60., 60.),
            dims: glm::to_dvec2(button_dims),
        };
        Ok(World {
            text,
            button,
            fps: 0.,
            cursor: glm::ivec2(0, 0),
        })
    }
}

impl engine::World for World {
    fn update(mut self, input: &input::State, _: Duration) -> engine::State<Self> {
        self.cursor = input.mouse_coords();
        engine::State::Running(self)
    }

    fn tick(mut self, time: &timer::GameTime) -> Self {
        self.fps = time.fps();
        self
    }
}

impl<'t, F, TL> IntoScene<Scene<TL::Texture>, fixed::State, Helper<'t, F, TL>> for World
where
    TL: FontTexturizer<'t, F>,
{
    fn try_into(
        &self,
        _: &fixed::State,
        helpers: &mut Helper<'t, F, TL>,
    ) -> Result<Scene<TL::Texture>> {
        let background = Rc::clone(&helpers.background);
        let fps = {
            let fps = format!("{}", self.fps as u32);
            helpers
                .texture_loader
                .texturize(&helpers.font, &fps, &ColorRGBA(255, 255, 0, 255))
        }?;
        let button = {
            let color = if self.button.contains(&glm::to_dvec2(self.cursor)) {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            helpers
                .texture_loader
                .texturize(&helpers.font, self.text, &color)
        }?;
        let button_tl = glm::to_ivec2(self.button.top_left);

        Ok(Scene {
            background,
            fps,
            button,
            button_tl,
        })
    }
}

struct Scene<T> {
    background: Rc<T>,
    fps: T,
    button: T,
    button_tl: glm::IVec2,
}

impl<T> renderer::Scene for Scene<T> {
    type Texture = T;

    fn show<'t, R>(&self, renderer: &mut R) -> Result<()>
    where
        R: ?Sized + Renderer<'t, Texture = Self::Texture>,
    {
        renderer.copy(&self.background, options::flip(options::Flip::Both))?;
        renderer.copy(&self.background, options::none())?;
        renderer.copy(&self.fps, options::at(align::top(0).right(1280)))?;
        renderer.copy(
            &self.button,
            options::at(self.button_tl).flip(options::Flip::Horizontal),
        )
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
    let font_loader = sdl2::ttf::init().unwrap();
    let texture_loader = canvas.texture_creator();

    let helper = Helper::load(&texture_loader, &font_loader).unwrap();
    let world = World::load(&helper.font).unwrap();
    let step = step::FixedUpdate::default().rate(30);
    let mut engine = Engine::new(event_pump, canvas, step);
    engine
        .run::<Scene<sdl2::render::Texture>, _, _>(world, helper)
        .unwrap();
}
