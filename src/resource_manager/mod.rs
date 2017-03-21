mod renderer;
mod frame_animator;
mod resource_loader;
mod tile_sheet;

pub use self::frame_animator::FrameAnimator;
pub use self::renderer::{BackEndRenderer, BackEndWindow, ImageDims, BackEnd};
pub use self::resource_loader::{BackEndLoader, ResourceLoader};
pub use self::tile_sheet::{Tile, TileSheet};

use window_wrapper::*;
use errors::*;

use std::collections::HashMap;
use std::cell::RefCell;

use glm;
use sdl2::rect;

pub trait Drawable {
    fn draw<R>(&self, dst_rect: glm::IVec4, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: BackEndRenderer;
}

pub trait Scene {
    fn show<R: BackEndRenderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()>;
}

#[derive(Copy,Clone,Hash,PartialEq,Eq,Debug)]
pub struct TextureId(pub usize);

#[derive(Copy,Clone)]
pub struct Texture {
    pub id: TextureId,
    pub dims: glm::UVec2,
}

pub struct ResourceManager<R: BackEnd> {
    pub wrap_coords: Option<glm::UVec2>,
    pub texture_cache: RefCell<HashMap<&'static str, Texture>>,
    pub data_cache: RefCell<HashMap<TextureId, R::Texture>>,
    pub renderer: R,
}

impl<R: BackEndWindow> ResourceManager<R> {
    pub fn output_size(&self) -> Result<glm::UVec2> {
        let (x, y) = self.renderer.output_size()?;
        Ok(glm::uvec2(x, y))
    }
}

impl<R: BackEnd> ResourceManager<R> {
    pub fn new(renderer: R) -> Self {
        ResourceManager {
            wrap_coords: None,
            texture_cache: RefCell::new(HashMap::new()),
            data_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }
}

impl<R: BackEndRenderer> ResourceManager<R> {
    pub fn draw(&mut self,
                id: TextureId,
                dst: Option<glm::IVec4>,
                src: Option<glm::UVec4>)
                -> Result<()> {
        match (dst, self.wrap_coords) {
            (Some(d), Some(w)) => self.draw_and_wrap(id, d, src, w),
            _ => self.draw_raw(id, dst, src),
        }
    }

    pub fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.renderer.fill_rects(rects)
    }

    pub fn clear(&mut self) {
        self.renderer.clear();
    }

    pub fn present(&mut self) {
        self.renderer.present();
    }

    pub fn show<S: Scene>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    pub fn render<D: Drawable>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }

    fn draw_and_wrap(&mut self,
                     id: TextureId,
                     dst: glm::IVec4,
                     src: Option<glm::UVec4>,
                     wrapping_coords: glm::UVec2)
                     -> Result<()> {
        wrap_rects(dst, wrapping_coords)
            .iter()
            .filter_map(|&r| r)
            .map(|r| self.draw_raw(id, Some(r), src))
            .fold(Ok(()), |res, x| if res.is_err() { res } else { x })
    }

    fn draw_raw(&mut self,
                id: TextureId,
                dst: Option<glm::IVec4>,
                src: Option<glm::UVec4>)
                -> Result<()> {
        let cache = self.data_cache.borrow();
        let texture = cache.get(&id).ok_or("texture not loaded")?;
        let src = src.map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = dst.map(Self::get_rect);
        self.renderer.copy(texture, src, dst)
    }

    fn get_rect(rect: glm::IVec4) -> rect::Rect {
        rect::Rect::new(rect.x, rect.y, rect.z as u32, rect.w as u32)
    }
}

impl Scene for TextureId {
    fn show<R: BackEndRenderer>(&self, renderer: &mut ResourceManager<R>) -> Result<()> {
        renderer.draw(*self, None, None).map_err(Into::into)
    }
}

impl Drawable for TextureId {
    fn draw<R>(&self, dst_rect: glm::IVec4, renderer: &mut ResourceManager<R>) -> Result<()>
        where R: BackEndRenderer
    {
        renderer.draw(*self, Some(dst_rect), None).map_err(Into::into)
    }
}