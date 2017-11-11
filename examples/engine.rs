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

struct HoverText {
    text: &'static str,
    body: Rectangle,
    is_hovering: bool,
}

struct HoverTextScene<T> {
    texture: T,
    top_left: glm::IVec2,
}

impl<'t, T, F, FT> IntoScene<HoverTextScene<T>, fixed::State, Helper<'t, F, FT>> for HoverText
where
    FT: FontTexturizer<'t, F, Texture = T>,
{
    fn try_into(
        &self,
        _: &fixed::State,
        helpers: &mut Helper<'t, F, FT>,
    ) -> Result<HoverTextScene<FT::Texture>> {
        let texture = {
            let color = if self.is_hovering {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            helpers
                .texture_loader
                .texturize(&helpers.font, self.text, &color)
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
    fps: f64,
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
        Ok(World { text, fps: 0. })
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
        fixed: &fixed::State,
        helpers: &mut Helper<'t, F, TL>,
    ) -> Result<Scene<TL::Texture>> {
        let background = Rc::clone(&helpers.background);
        let fps = {
            let fps = format!("{}", self.fps as u32);
            helpers
                .texture_loader
                .texturize(&helpers.font, &fps, &ColorRGBA(255, 255, 0, 255))
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
