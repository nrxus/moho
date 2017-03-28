use errors::*;

use sdl2::ttf::{self, Font, Sdl2TtfContext};

pub struct FontManager {
    context: Sdl2TtfContext,
}

impl FontManager {
    pub fn init() -> Result<Self> {
        let context = ttf::init().chain_err(|| "failed to initialize font context")?;
        let manager = FontManager { context: context };
        Ok(manager)
    }

    pub fn load(&self, path: &str, size: u16) -> Result<Font> {
        self.context.load_font(path, size).map_err(Into::into)
    }
}
