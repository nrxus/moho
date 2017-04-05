mod frame_animator;
mod font;
mod resource_manager;
mod tile_sheet;
mod sdl2;

pub use self::font::*;
pub use self::frame_animator::FrameAnimator;
pub use self::resource_manager::ResourceManager;
pub use self::tile_sheet::{Tile, TileSheet};

use errors::*;

use glm;
use sdl2::rect;

pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

pub trait Resource {}
pub trait Texture: Resource {
    fn dims(&self) -> glm::UVec2;
}

pub trait Loader<'a, T: Resource, D: ?Sized> {
    fn load(&'a self, data: &D) -> Result<T>;
}

pub trait ResourceLoader
    : for<'a> Loader<'a, <Self as ResourceLoader>::Texture, str> {
    type Texture: Texture;
}

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

pub trait Drawable<R: ?Sized> {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: ?Sized> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait Renderer {
    type Texture;
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
