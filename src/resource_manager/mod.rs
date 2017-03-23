mod frame_animator;
mod window;
mod renderer;
mod resource_loader;
mod tile_sheet;

pub use self::frame_animator::FrameAnimator;
pub use self::window::Window;
pub use self::renderer::Renderer;
pub use self::resource_loader::ResourceLoader;
pub use self::tile_sheet::{Tile, TileSheet};

use self::window::BackEndWindow;

use errors::*;

use std::collections::HashMap;
use std::cell::RefCell;

use glm;
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

impl BackEnd for SdlRenderer<'static> {
    type Texture = SdlTexture;
}

pub trait Drawable {
    fn draw<R: Renderer>(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene {
    fn show<R: Renderer>(&self, renderer: &mut R) -> Result<()>;
}

#[derive(Copy,Clone,Hash,PartialEq,Eq,Debug)]
pub struct TextureId(pub usize);

#[derive(Copy,Clone)]
pub struct Texture {
    pub id: TextureId,
    pub dims: glm::UVec2,
}

pub struct ResourceManager<R: BackEnd> {
    pub texture_cache: RefCell<HashMap<String, Texture>>,
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
            texture_cache: RefCell::new(HashMap::new()),
            data_cache: RefCell::new(HashMap::new()),
            renderer: renderer,
        }
    }
}

impl Scene for TextureId {
    fn show<R: Renderer>(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(*self, None, None)
    }
}

impl Drawable for TextureId {
    fn draw<R: Renderer>(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()> {
        renderer.draw(*self, Some(dst_rect), None)
    }
}
