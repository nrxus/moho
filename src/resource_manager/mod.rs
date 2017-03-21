mod backend;
mod frame_animator;
mod window;
mod renderer;
mod resource_loader;
mod tile_sheet;

pub use self::frame_animator::FrameAnimator;
pub use self::backend::{ImageDims, BackEnd};
pub use self::window::{BackEndWindow, Window};
pub use self::renderer::{BackEndRenderer, Renderer};
pub use self::resource_loader::{BackEndLoader, ResourceLoader};
pub use self::tile_sheet::{Tile, TileSheet};

use errors::*;

use std::collections::HashMap;
use std::cell::RefCell;

use glm;

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
