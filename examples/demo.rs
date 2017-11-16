extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::input;
use moho::renderer::*;
use moho::shape::*;
use moho::timer::*;

pub struct MainGame<'f, 't, TL: 't, FL: 'f, R, E>
where
    TL: TextureLoader<'t>,
    FL: FontLoader<'f>,
{
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, TL>,
    font_manager: font::Manager<'f, FL>,
    renderer: R,
}

impl<'f, 't, TL, FL, R, E> MainGame<'f, 't, TL, FL, R, E>
where
    E: input::EventPump,
    TL: TextureLoader<'t>,
    TL::Texture: Texture,
    FL: FontLoader<'f>,
    FL::Font: Font<Texture = TL::Texture>,
{
    pub fn new(renderer: R, event_pump: E, font_loader: &'f FL, texture_loader: &'t TL) -> Self {
        let texture_manager = TextureManager::new(texture_loader);
        let font_manager = font::Manager::new(font_loader);
        let input_manager = input::Manager::new(event_pump);
        MainGame {
            input_manager: input_manager,
            texture_manager: texture_manager,
            font_manager: font_manager,
            renderer: renderer,
        }
    }

    pub fn run(&mut self) -> Result<()>
    where
        R: Canvas<'t, Texture = <TL as TextureLoader<'t>>::Texture>,
    {
        let image = self.texture_manager.load("examples/background.png")?;
        let font_details = font::Details {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let button_text = "HOVER ON ME";
        let font = self.font_manager.load(&font_details)?;
        let button_dims = font.measure(button_text)?;

        let button_tl = glm::ivec2(60, 60);
        let rect = Rectangle {
            top_left: glm::to_dvec2(button_tl),
            dims: glm::to_dvec2(button_dims),
        };
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            let state = self.input_manager.update();
            let cursor_position = state.mouse_coords();
            let color = if rect.contains(&glm::to_dvec2(cursor_position)) {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            let button_texture = font.texturize(button_text, &color)?;
            let fps = format!("{}", game_time.fps() as u32);
            let font_texture = font.texturize(&fps, &ColorRGBA(255, 255, 0, 255))?;
            self.renderer.clear();
            self.renderer
                .copy(&image, options::flip(options::Flip::Both))?;
            self.renderer.copy(&image, options::none())?;
            self.renderer
                .copy(&font_texture, options::at(align::top(0).right(1280)))?;
            self.renderer.copy(
                &button_texture,
                options::at(button_tl).flip(options::Flip::Horizontal),
            )?;
            self.renderer.present();
        }
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.current.game_quit()
    }
}


fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let name = "MohoGame";

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

    let mut game = MainGame::new(canvas, event_pump, &font_loader, &texture_loader);
    game.run().unwrap();
}
