use errors::*;

use glm;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

pub trait ImageDims {
    fn dims(&self) -> glm::UVec2;
}

impl ImageDims for SdlTexture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

pub trait BackEnd {
    type Texture: ImageDims;
}

pub trait BackEndRenderer: BackEnd {
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()>;
}

pub trait BackEndWindow: BackEnd {
    fn output_size(&self) -> Result<(u32, u32)>;
}

impl BackEnd for SdlRenderer<'static> {
    type Texture = SdlTexture;
}

impl BackEndWindow for SdlRenderer<'static> {
    fn output_size(&self) -> Result<(u32, u32)> {
        Ok(self.output_size()?)
    }
}

impl BackEndRenderer for SdlRenderer<'static> {
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
