use super::{Drawable, Scene, ResourceManager, TextureId};
use window_wrapper::*;
use errors::*;

use glm;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

use std::cell::{Ref, RefCell};
use std::collections::HashMap;

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
        match (dst, self.wrap_coords) {
            (Some(d), Some(w)) => {
                draw_and_wrap(&self.data_cache, id, d, src, w, &mut self.renderer)
            }
            _ => draw_raw(self.data_cache.borrow(), id, dst, src, &mut self.renderer),
        }
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

fn draw_and_wrap<R: BackEndRenderer>(cache: &RefCell<HashMap<TextureId, R::Texture>>,
                                     id: TextureId,
                                     dst: glm::IVec4,
                                     src: Option<glm::UVec4>,
                                     wrapping_coords: glm::UVec2,
                                     renderer: &mut R)
                                     -> Result<()> {
    wrap_rects(dst, wrapping_coords)
        .iter()
        .filter_map(|&r| r)
        .map(|r| draw_raw(cache.borrow(), id, Some(r), src, renderer))
        .fold(Ok(()), |res, x| if res.is_err() { res } else { x })
}

fn draw_raw<R: BackEndRenderer>(cache: Ref<HashMap<TextureId, R::Texture>>,
                                id: TextureId,
                                dst: Option<glm::IVec4>,
                                src: Option<glm::UVec4>,
                                renderer: &mut R)
                                -> Result<()> {
    let texture = cache.get(&id).ok_or("texture not loaded")?;
    let src = src.map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
    let dst = dst.map(|r| rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32));
    renderer.copy(texture, src, dst)
}
