use super::{Drawable, Scene, ResourceManager, TextureId};
use errors::*;

use glm;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

pub trait BackEndRenderer: super::BackEnd {
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()>;
}

impl BackEndRenderer for SdlRenderer<'static> {
    fn copy(&mut self,
            texture: &SdlTexture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()> {
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
}

pub trait Renderer {
    fn draw(&mut self,
            id: TextureId,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()>;
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn clear(&mut self);
    fn present(&mut self);
    fn show<S: Scene>(&mut self, scene: &S) -> Result<()>;
    fn render<D: Drawable>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()>;
}

impl<R: BackEndRenderer> Renderer for ResourceManager<R> {
    fn draw(&mut self,
            id: TextureId,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()> {
        let cache = self.data_cache.borrow();
        let texture = cache.get(&id).ok_or("texture not loaded")?;
        let src = src.map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = dst.map(|r| rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32));
        self.renderer.copy(texture, src, dst)
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.renderer.fill_rects(rects)
    }

    fn clear(&mut self) {
        self.renderer.clear();
    }

    fn present(&mut self) {
        self.renderer.present();
    }

    fn show<S: Scene>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn render<D: Drawable>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}
