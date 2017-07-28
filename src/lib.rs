#[macro_use]
extern crate error_chain;
extern crate glm;
extern crate num_traits;
extern crate sdl2;

use sdl2::render::WindowCanvas as SdlCanvas;
use sdl2::render::TextureCreator;
use sdl2::video::WindowContext;
use sdl2::EventPump as SdlEventPump;
use sdl2::image::{INIT_JPG, INIT_PNG};

pub mod animation;
pub mod input;
pub mod shape;
pub mod renderer;
pub mod timer;
pub mod window_wrapper;

pub mod errors {
    error_chain!{}
}

error_chain!{
    foreign_links {
        WindowBuild(sdl2::video::WindowBuildError);
        SdlContext(sdl2::IntegerOrSdlError);
        TtfInit(sdl2::ttf::InitError);
        TextureCreateError(sdl2::render::TextureValueError);
    }
}

pub fn init(
    name: &'static str,
    width: u32,
    height: u32,
) -> Result<
    (
        SdlCanvas,
        TextureCreator<WindowContext>,
        input::Manager<SdlEventPump>,
    ),
> {
    let sdl_ctx = sdl2::init()?;
    let video_ctx = sdl_ctx.video()?;
    let _image_ctx = sdl2::image::init(INIT_PNG | INIT_JPG)?;

    let window = video_ctx
        .window(name, width, height)
        .position_centered()
        .opengl()
        .build()?;

    let mut renderer = window.into_canvas().present_vsync().build()?;
    let creator = renderer.texture_creator();

    renderer.clear();
    renderer.present();

    let event_pump = sdl_ctx.event_pump()?;
    let input_manager = input::Manager::new(event_pump);
    Ok((renderer, creator, input_manager))
}
