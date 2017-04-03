use errors::*;
use renderer;

use glm;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::ttf::Font as SdlFont;
use sdl2::ttf::Sdl2TtfContext;

impl renderer::Resource for SdlTexture {}
impl<'a> renderer::Font for SdlFont<'a, 'static> {}
impl<'a> renderer::Resource for SdlFont<'a, 'static> {}

impl renderer::Texture for SdlTexture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

impl<'a> renderer::FontLoader<'a, SdlFont<'a, 'static>> for Sdl2TtfContext {}

impl<'a> renderer::ResourceLoader<'a, SdlTexture> for SdlRenderer<'static> {}

impl<'a> renderer::Loader<'a, SdlFont<'a, 'static>> for Sdl2TtfContext {
    type LoadData = renderer::FontDetails;
    fn load(&'a self, data: &renderer::FontDetails) -> Result<SdlFont<'a, 'static>> {
        self.load_font(data.path, data.size).map_err(Into::into)
    }
}

impl<'a> renderer::Loader<'a, SdlTexture> for SdlRenderer<'static> {
    type LoadData = str;
    fn load(&'a self, path: &str) -> Result<SdlTexture> {
        self.load_texture(path).map_err(Into::into)
    }
}

impl<'a> renderer::FontTexturizer<'a, SdlFont<'a, 'static>, Sdl2TtfContext>
    for
    SdlRenderer<'static> {
    type Texture = SdlTexture;
    fn texturize(&self,
                 font: &SdlFont<'a, 'static>,
                 text: &str,
                 color: renderer::ColorRGBA)
                 -> Result<SdlTexture> {
        let renderer::ColorRGBA(red, green, blue, alpha) = color;
        let color = Color::RGBA(red, green, blue, alpha);
        let surface = font.render(text)
            .blended(color)
            .chain_err(|| "error when creatinga a blended font surface")?;

        self.create_texture_from_surface(&surface)
            .chain_err(|| "error creating a texture from a surface")
    }
}

impl renderer::Renderer<SdlTexture> for SdlRenderer<'static> {
    fn copy(&mut self,
            texture: &SdlTexture,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()> {
        let src = src.map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = dst.map(|r| rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32));
        self.copy(texture, src, dst).map_err(Into::into)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.fill_rects(rects).map_err(Into::into)
    }

    fn show<S: renderer::Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn render<D: renderer::Drawable<Self>>(&mut self,
                                           drawable: &D,
                                           dst_rect: glm::IVec4)
                                           -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}

impl renderer::Window for SdlRenderer<'static> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size()?;
        Ok(glm::uvec2(width, height))
    }
}
