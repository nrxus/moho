extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::MohoEngine;
use moho::input_manager::*;
use moho::resource_manager::*;
use moho::timer::*;

use sdl2::ttf::Font;

pub struct MainGame<E: MohoEngine> {
    input_manager: InputManager<E::EventPump>,
    renderer: E::Renderer,
}

impl<E: MohoEngine> MainGame<E> {
    pub fn new(renderer: E::Renderer, input_manager: InputManager<E::EventPump>) -> Self {
        MainGame {
            input_manager: input_manager,
            renderer: renderer,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        let font_manager = FontManager::init()?;
        let font = font_manager.load("examples/fonts/kenpixel_mini.ttf", 48)?;
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            self.input_manager.update();
            self.draw(&font, game_time.fps())?;
        }
        Ok(())
    }

    fn draw(&mut self, font: &Font, fps: f64) -> Result<()> {
        self.renderer.clear();
        let fps = format!("{}", fps as u32);
        let texture = self.renderer.texturize(font, &fps)?;
        self.renderer.draw_texture(&texture)?;
        self.renderer.present();
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
    let mut game = MainGame::<moho::SdlMohoEngine>::new(renderer, input_manager);
    game.run().unwrap();
}
