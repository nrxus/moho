extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::MohoEngine;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::timer::*;

pub struct MainGame<'ttf, E: MohoEngine<'ttf>> {
    input_manager: InputManager<E::EventPump>,
    font_manager: FontManager<E::FontLoader>,
    resource_manager: ResourceManager<E::Renderer>,
    renderer: E::Renderer,
}

impl<'ttf, E: MohoEngine<'ttf>> MainGame<'ttf, E> {
    pub fn new(renderer: E::Renderer,
               input_manager: InputManager<E::EventPump>,
               font_loader: E::FontLoader)
               -> Self {
        let font_manager = FontManager::init(font_loader);
        let resource_manager = ResourceManager::new();
        MainGame {
            input_manager: input_manager,
            font_manager: font_manager,
            resource_manager: resource_manager,
            renderer: renderer,
        }
    }

    pub fn run(&'ttf mut self) -> Result<()> {
        let image = self.resource_manager.load_texture("examples/background.png", &self.renderer)?;
        let font = self.font_manager.load("examples/fonts/kenpixel_mini.ttf", 48)?;
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            self.input_manager.update();
            self.renderer.clear();
            let fps = format!("{}", game_time.fps() as u32);
            let font_texture = self.renderer.texturize(&font, &fps)?;
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
    let mut game = MainGame::<moho::SdlMohoEngine>::new(renderer, input_manager, loader);
    game.run().unwrap();
}
