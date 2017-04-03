extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::input_manager::*;
use moho::renderer::*;
use moho::timer::*;

pub struct MainGame<'ttf, E: EventPump, T, R, FL: 'ttf> {
    input_manager: InputManager<E>,
    resource_manager: ResourceManager<T, String>,
    renderer: R,
    font_loader: &'ttf FL,
}

impl<'ttf, E: EventPump, T: Texture, R, FL> MainGame<'ttf, E, T, R, FL> {
    pub fn new(renderer: R, input_manager: InputManager<E>, font_loader: &'ttf FL) -> Self {
        let resource_manager = ResourceManager::new();
        MainGame {
            input_manager: input_manager,
            resource_manager: resource_manager,
            renderer: renderer,
            font_loader: font_loader,
        }
    }

    pub fn run<F: Font>(&mut self) -> Result<()>
        where FL: FontLoader<'ttf, F>,
          R: for<'a> ResourceLoader<'a, T> + Renderer<T> + FontTexturizer<'ttf, F, FL, Texture = T>
    {
        let image = self.resource_manager.load("examples/background.png", &self.renderer)?;
        let font_details = FontDetails {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let font = self.font_loader.load(&font_details)?;
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            self.input_manager.update();
            self.renderer.clear();
            let fps = format!("{}", game_time.fps() as u32);
            let font_texture = self.renderer.texturize(&font, &fps, ColorRGBA(255, 255, 0, 255))?;
            let font_dst = glm::ivec4(0,
                                      0,
                                      font_texture.dims().x as i32,
                                      font_texture.dims().y as i32);
            self.renderer.copy(&image, None, None)?;
            self.renderer.copy(&font_texture, Some(font_dst), None)?;
            self.renderer.present();
        }
        Ok(())
    }

    fn game_quit(&self) -> bool {
        self.input_manager.game_quit()
    }
}


fn main() {
    const WINDOW_WIDTH: u32 = 1280;
    const WINDOW_HEIGHT: u32 = 720;
    let (renderer, input_manager) = moho::init("MohoGame", WINDOW_WIDTH, WINDOW_HEIGHT).unwrap();
    let loader = sdl2::ttf::init().chain_err(|| "cannot init loader").unwrap();
    let mut game = MainGame::new(renderer, input_manager, &loader);
    game.run().unwrap();
}
