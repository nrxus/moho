use errors::*;
use super::FontLoader;

pub struct FontManager<L> {
    loader: L,
}

impl<'a, L: FontLoader<'a>> FontManager<L> {
    pub fn init(loader: L) -> Self {
        FontManager { loader: loader }
    }

    pub fn load(&'a self, path: &str, size: u16) -> Result<L::Font> {
        self.loader.load(path, size).map_err(Into::into)
    }
}
