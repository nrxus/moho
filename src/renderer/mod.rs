mod frame_animator;
mod resource_manager;
mod tile_sheet;
mod sdl2;

pub use self::frame_animator::FrameAnimator;
pub use self::resource_manager::ResourceManager;
pub use self::tile_sheet::{Tile, TileSheet};

use errors::*;

use glm;
use sdl2::rect;

pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

pub trait Resource {}
pub trait Font: Resource {}
pub trait Texture: Resource {
    fn dims(&self) -> glm::UVec2;
}

pub trait Loader<'a, T: Resource> {
    type LoadData: ?Sized;
    fn load(&'a self, data: &Self::LoadData) -> Result<T>;
}

#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: &'static str,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path,
            size: details.size,
        }
    }
}

pub trait ResourceLoader<'a, T: Texture>: Loader<'a, T, LoadData = str> {}

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

pub trait Drawable<R> {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait FontLoader<'a, T: Font>: Loader<'a, T, LoadData = FontDetails> {}

pub trait Renderer<T: Texture>: Sized {
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &T,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()>;
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()>;
    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()>;
}

pub trait FontTexturizer<'a, F: Font, L: FontLoader<'a, F>> {
    type Texture: Texture;
    fn texturize(&self, font: &F, text: &str, color: ColorRGBA) -> Result<Self::Texture>;
}
