use errors::*;
use resource_manager::{BackEnd, Drawable, FontTexturizer, ImageDims, Scene, Renderer,
                       ResourceLoader, Window};

use glm;
use sdl2::image::LoadTexture;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::ttf::Font;

use std::path::Path;

impl ImageDims for SdlTexture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

impl BackEnd for SdlRenderer<'static> {
    type Texture = SdlTexture;
}

impl ResourceLoader for SdlRenderer<'static> {
    fn load_texture(&self, path: &Path) -> Result<SdlTexture> {
        LoadTexture::load_texture(self, path).map_err(Into::into)
    }
}

impl FontTexturizer for SdlRenderer<'static> {
    fn texturize(&self, font: &Font, text: &str) -> Result<SdlTexture> {
        let surface = font.render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .chain_err(|| "error when creatinga a blended font surface")?;

        self.create_texture_from_surface(&surface)
            .chain_err(|| "error creating a texture from a surface")
    }
}

impl Renderer for SdlRenderer<'static> {
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

    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}

impl Window for SdlRenderer<'static> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size()?;
        Ok(glm::uvec2(width, height))
    }
}
