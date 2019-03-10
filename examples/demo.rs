use moho::{
    font::{self, Font},
    input,
    renderer::{align, options, Canvas, ColorRGBA, Draw, Position},
    shape::*,
    texture::{self, Texture},
    timer::*,
};

pub struct MainGame<'f, 't, TL, FL, R, E>
where
    TL: texture::Loader<'t>,
    FL: font::Loader<'f>,
{
    input_manager: input::Manager<E>,
    texture_manager: texture::Manager<'t, TL>,
    font_manager: font::Manager<'f, FL>,
    renderer: R,
}

impl<'f, 't, TL, FL, R, E> MainGame<'f, 't, TL, FL, R, E>
where
    E: input::EventPump,
    TL: texture::Loader<'t>,
    TL::Texture: Texture + Draw<R>,
    FL: font::Loader<'f>,
    FL::Font: Font<Texture = TL::Texture>,
    R: Canvas,
{
    pub fn new(renderer: R, event_pump: E, font_loader: &'f FL, texture_loader: &'t TL) -> Self {
        let texture_manager = texture::Manager::new(texture_loader);
        let font_manager = font::Manager::new(font_loader);
        let input_manager = input::Manager::new(event_pump);
        MainGame {
            input_manager,
            texture_manager,
            font_manager,
            renderer,
        }
    }

    pub fn run(&mut self) -> Result<(), failure::Error> {
        let image = self.texture_manager.load("examples/background.png")?;
        let font_details = font::Details {
            path: "examples/fonts/kenpixel_mini.ttf",
            size: 48,
        };
        let font = self.font_manager.load(&font_details)?;
        let button_text = "HOVER ON ME";
        let button_dims = font.measure(button_text)?;

        let button_tl = glm::ivec2(60, 60);
        let rect = Rectangle {
            top_left: glm::to_dvec2(button_tl),
            dims: glm::to_dvec2(button_dims),
        };
        let mut timer = Timer::new();
        loop {
            match self.input_manager.update() {
                moho::State::Quit(_) => {
                    break;
                }
                moho::State::Running(input) => Self::process(
                    input,
                    &mut timer,
                    &rect,
                    &font,
                    button_text,
                    &*image,
                    &mut self.renderer,
                )?,
            }
        }
        Ok(())
    }

    fn process(
        input: &input::State,
        timer: &mut Timer,
        rect: &Rectangle,
        font: &FL::Font,
        button_text: &str,
        image: &TL::Texture,
        renderer: &mut R,
    ) -> Result<(), failure::Error> {
        let game_time = timer.update();
        let cursor_position = input.mouse_coords();
        let color = if rect.contains(&glm::to_dvec2(cursor_position)) {
            ColorRGBA(255, 0, 0, 255)
        } else {
            ColorRGBA(255, 255, 0, 255)
        };
        let button_texture = font.texturize(button_text, color)?;
        let button_dst = Position::from(glm::to_ivec2(rect.top_left)).dims(button_texture.dims());
        let fps = format!("{}", game_time.fps() as u32);
        let font_texture = font.texturize(&fps, ColorRGBA(255, 255, 0, 255))?;
        let font_dst = align::top(0).right(1280).dims(font_texture.dims());
        renderer.clear();
        renderer.draw(image, options::flip(options::Flip::Both))?;
        renderer.draw(image, options::none())?;
        renderer.draw(&font_texture, options::at(font_dst))?;
        renderer.draw(
            &button_texture,
            options::at(button_dst).flip(options::Flip::Horizontal),
        )?;
        renderer.present();
        Ok(())
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
    let font_loader = moho::sdl2_helpers::font::Loader::load(&texture_loader).unwrap();

    let mut game = MainGame::new(canvas, event_pump, &font_loader, &texture_loader);
    game.run().unwrap();
}
