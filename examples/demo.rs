extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::input;
use moho::renderer::*;
use moho::shape::*;
use moho::timer::*;

pub struct MainGame<'f, 't, TL: 't, FL: 'f, R, E>
    where TL: TextureLoader<'t>,
          FL: FontLoader<'f>
{
    input_manager: input::Manager<E>,
    texture_manager: TextureManager<'t, TL>,
    font_manager: FontManager<'f, FL>,
    renderer: R,
    texture_loader: &'t TL,
}

impl<'f, 't, TL, FL, R, E> MainGame<'f, 't, TL, FL, R, E>
    where TL: TextureLoader<'t>,
          FL: FontLoader<'f>
{
    pub fn new(renderer: R,
               input_manager: input::Manager<E>,
               font_loader: &'f FL,
               texture_loader: &'t TL)
               -> Self {
        let texture_manager = TextureManager::new(texture_loader);
        let font_manager = FontManager::new(font_loader);
        MainGame {
            input_manager: input_manager,
            texture_manager: texture_manager,
            font_manager: font_manager,
            renderer: renderer,
            texture_loader: texture_loader,
        }
    }

    pub fn run(&mut self) -> Result<()>
        where TL: FontTexturizer<'f,
                                 't,
                                 Texture = <TL as TextureLoader<'t>>::Texture,
                                 Font = FL::Font>,
              R: Renderer<'t, Texture = <TL as TextureLoader<'t>>::Texture>,
              E: input::EventPump
    {
        let image = self.texture_manager.load("examples/background.png")?;
        let font_details = FontDetails {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let button_text = "HOVER ON ME";
        let font = self.font_manager.load(&font_details)?;
        let button_dims = font.measure(button_text)?;

        let rect = Rectangle {
            top_left: glm::dvec2(60., 60.),
            dims: glm::to_dvec2(button_dims),
        };
        let button_dst = glm::ivec4(60, 60, button_dims.x as i32, button_dims.y as i32);
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
            let button_texture = self.texture_loader
                .texturize(&font, button_text, &color)?;
            let fps = format!("{}", game_time.fps() as u32);
            let font_texture = self.texture_loader
                .texturize(&font, &fps, &ColorRGBA(255, 255, 0, 255))?;
            let font_dst = glm::ivec4(0,
                                      0,
                                      font_texture.dims().x as i32,
                                      font_texture.dims().y as i32);
            self.renderer.clear();
            self.renderer.copy(&image, None, None)?;
            self.renderer
                .copy(&font_texture, Some(&font_dst), None)?;
            self.renderer
                .copy(&button_texture, Some(&button_dst), None)?;
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
    let (renderer, creator, input_manager) = moho::init("MohoGame", WINDOW_WIDTH, WINDOW_HEIGHT)
        .unwrap();
    let font_loader = sdl2::ttf::init()
        .chain_err(|| "cannot init loader")
        .unwrap();
    let mut game = MainGame::new(renderer, input_manager, &font_loader, &creator);
    game.run().unwrap();
}
