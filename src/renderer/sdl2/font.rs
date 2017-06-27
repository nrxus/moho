use errors::*;
use renderer;
use renderer::font;

use glm;
use sdl2::pixels::Color;
use sdl2::render;
use sdl2::ttf::Font as SdlFont;
use sdl2::ttf::Sdl2TtfContext;

impl<'a> font::Font for SdlFont<'a, 'static> {
    fn measure(&self, text: &str) -> Result<glm::UVec2> {
        self.size_of(text)
            .map(|(x, y)| glm::uvec2(x, y))
            .chain_err(|| "error measuring font")
    }
}

impl<'a> renderer::FontLoader<'a> for Sdl2TtfContext {
    type Font = SdlFont<'a, 'static>;
}

impl<'a> renderer::Loader<'a, SdlFont<'a, 'static>> for Sdl2TtfContext {
    type Args = renderer::FontDetails;
    fn load(&'a self, data: &renderer::FontDetails) -> Result<SdlFont<'a, 'static>> {
        self.load_font(data.path, data.size).map_err(Into::into)
    }
}

impl<'t, 'f, T> renderer::FontTexturizer<'t, SdlFont<'f, 'static>> for render::TextureCreator<T> {
    type Texture = render::Texture<'t>;
    fn texturize(
        &'t self,
        font: &SdlFont<'f, 'static>,
        text: &str,
        color: &renderer::ColorRGBA,
    ) -> Result<Self::Texture> {
        let &renderer::ColorRGBA(red, green, blue, alpha) = color;
        let color = Color::RGBA(red, green, blue, alpha);
        let surface = font.render(text)
            .blended(color)
            .chain_err(|| "error when creatinga a blended font surface")?;

        self.create_texture_from_surface(&surface)
            .chain_err(|| "error creating a texture from a surface")
    }
}
