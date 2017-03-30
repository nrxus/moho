mod frame_animator;
mod resource_loader;
mod tile_sheet;
mod sdl2;

pub use self::frame_animator::FrameAnimator;
pub use self::resource_loader::ResourceManager;
pub use self::tile_sheet::{Tile, TileSheet};

use errors::*;

use glm;
use sdl2::rect;

use std::path::Path;

pub trait ImageDims {
    fn dims(&self) -> glm::UVec2;
}

pub trait BackEnd {
    type Texture: ImageDims;
}

pub trait ResourceLoader: BackEnd {
    fn load_texture(&self, path: &Path) -> Result<Self::Texture>;
}

pub trait Window: BackEnd {
    fn output_size(&self) -> Result<glm::UVec2>;
}

pub trait Drawable<R> {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait FontLoader<'a> {
    type Font;
    fn load(&'a self, path: &str, size: u16) -> Result<Self::Font>;
}

pub trait Renderer: BackEnd + Sized {
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()>;
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()>;
    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()>;
}

pub trait FontTexturizer<'a, L: FontLoader<'a>>: BackEnd {
    fn texturize(&self, font: &L::Font, text: &str) -> Result<Self::Texture>;
}
