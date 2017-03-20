use errors::*;

use glm;
use sdl2::rect;
use sdl2::image::LoadTexture;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

use std::path::Path;

pub trait ImageDims {
    fn dims(&self) -> glm::UVec2;
}

impl ImageDims for SdlTexture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

pub trait Renderer {
    type Texture: ImageDims;

    fn load_texture(&self, path: &Path) -> Result<Self::Texture>;
    fn output_size(&self) -> Result<(u32, u32)>;

    // Drawing methods
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()>;
}

impl Renderer for SdlRenderer<'static> {
    type Texture = SdlTexture;

    fn load_texture(&self, path: &Path) -> Result<SdlTexture> {
        Ok(LoadTexture::load_texture(self, path)?)
    }

    fn output_size(&self) -> Result<(u32, u32)> {
        Ok(self.output_size()?)
    }

    fn copy(&mut self,
            texture: &SdlTexture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()> {
        Ok(self.copy(texture, src, dst)?)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        Ok(self.fill_rects(rects)?)
    }
}
