use font as moho;
use {renderer, resource, Result};

use failure;
use glm;
use sdl2::pixels::Color;
use sdl2::render;
use sdl2::ttf::Font as SdlFont;
use sdl2::ttf::{self, Sdl2TtfContext};

pub struct Font<'t, 'f, T: 't> {
    inner: SdlFont<'f, 'static>,
    creator: &'t render::TextureCreator<T>,
}

impl<'t, 'f, T> moho::Font for Font<'t, 'f, T> {
    type Texture = render::Texture<'t>;

    fn measure(&self, text: &str) -> Result<glm::UVec2> {
        self.inner
            .size_of(text)
            .map(|(x, y)| glm::uvec2(x, y))
            .map_err(failure::err_msg)
    }

    fn texturize(&self, text: &str, color: &renderer::ColorRGBA) -> Result<Self::Texture> {
        let &renderer::ColorRGBA(red, green, blue, alpha) = color;
        let color = Color::RGBA(red, green, blue, alpha);
        let surface = self.inner.render(text).blended(color)?;
        self.creator
            .create_texture_from_surface(&surface)
            .map_err(Into::into)
    }
}

pub struct Loader<'t, T: 't> {
    inner: Sdl2TtfContext,
    creator: &'t render::TextureCreator<T>,
}

impl<'t, T> Loader<'t, T> {
    pub fn load(creator: &'t render::TextureCreator<T>) -> Result<Self> {
        ttf::init()
            .map(|inner| Loader { inner, creator })
            .map_err(Into::into)
    }
}

impl<'f, 't, T> moho::Loader<'f> for Loader<'t, T> {
    type Font = Font<'t, 'f, T>;
}

impl<'f, 't, T> resource::Loader<'f, Font<'t, 'f, T>> for Loader<'t, T> {
    type Args = moho::Details;

    fn load(&'f self, data: &moho::Details) -> Result<Font<'t, 'f, T>> {
        self.inner
            .load_font(data.path, data.size)
            .map(|inner| Font {
                inner,
                creator: self.creator,
            })
            .map_err(failure::err_msg)
    }
}
