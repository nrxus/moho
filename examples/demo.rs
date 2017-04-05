extern crate glm;
extern crate moho;
extern crate sdl2;

use moho::errors::*;
use moho::input_manager::*;
use moho::renderer::*;
use moho::shape::*;
use moho::timer::*;

pub struct MainGame<'ttf, E: EventPump, T, R, FL: 'ttf, F> {
    input_manager: InputManager<E>,
    texture_manager: ResourceManager<String, T>,
    font_manager: ResourceManager<FontDetails, F>,
    renderer: R,
    font_loader: &'ttf FL,
}

impl<'ttf, E: EventPump, T: Texture, R, FL, F: Font> MainGame<'ttf, E, T, R, FL, F> {
    pub fn new(renderer: R, input_manager: InputManager<E>, font_loader: &'ttf FL) -> Self {
        let texture_manager = ResourceManager::new();
        let font_manager = ResourceManager::new();
        MainGame {
            input_manager: input_manager,
            texture_manager: texture_manager,
            font_manager: font_manager,
            renderer: renderer,
            font_loader: font_loader,
        }
    }

    pub fn run(&mut self) -> Result<()>
        where R: ResourceLoader<Texture = T> + Renderer<Texture = T>,
              R: FontTexturizer<'ttf, Font = F, Texture = T>,
              FL: FontLoader<'ttf, Font = F>
    {
        let image = self.texture_manager
            .load("examples/background.png", &self.renderer)?;
        let font_details = FontDetails {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let button_text = "HOVER ON ME";
        let font = self.font_manager.load(&font_details, self.font_loader)?;
        let button_dims = font.measure(button_text)?;

        let rect = Rectangle {
            top_left: glm::dvec2(60., 60.),
            dims: glm::to_dvec2(button_dims),
        };
        let button_dst = glm::ivec4(60, 60, button_dims.x as i32, button_dims.y as i32);
        let mut timer = Timer::new();
        while !self.game_quit() {
            let game_time = timer.update();
            self.input_manager.update();
            let cursor_position = self.input_manager.mouse_coords();
            let color = if rect.contains(&glm::to_dvec2(cursor_position)) {
                ColorRGBA(255, 0, 0, 255)
            } else {
                ColorRGBA(255, 255, 0, 255)
            };
            let button_texture = self.renderer.texturize(&font, button_text, color)?;
            let fps = format!("{}", game_time.fps() as u32);
            let font_texture = self.renderer
                .texturize(&font, &fps, ColorRGBA(255, 255, 0, 255))?;
            let font_dst = glm::ivec4(0,
                                      0,
                                      font_texture.dims().x as i32,
                                      font_texture.dims().y as i32);
            self.renderer.clear();
            self.renderer.copy(&image, None, None)?;
            self.renderer.copy(&font_texture, Some(font_dst), None)?;
            self.renderer
                .copy(&button_texture, Some(button_dst), None)?;
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
    let loader = sdl2::ttf::init()
        .chain_err(|| "cannot init loader")
        .unwrap();
    let mut game = MainGame::new(renderer, input_manager, &loader);
    game.run().unwrap();
}
