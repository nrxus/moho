mod font;
mod resource_manager;
mod sdl2;

pub use self::font::*;
pub use self::resource_manager::{FontManager, ResourceManager, TextureManager};

use errors::*;

use glm;
use sdl2::rect;

#[derive(Clone, Copy, Debug)]
pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

pub trait Texture {
    fn dims(&self) -> glm::UVec2;
}

pub trait Loader<'a, T> {
    type Args: ?Sized;
    fn load(&'a self, data: &Self::Args) -> Result<T>;
}

pub trait ResourceLoader<'a>
    : Loader<'a, <Self as ResourceLoader<'a>>::Texture, Args = str> {
    type Texture: Texture;
}

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

pub trait Drawable<R: ?Sized> {
    fn draw(&self, dst_rect: &glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: ?Sized> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait Renderer<'t> {
    type Texture;

    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            dst: Option<&glm::IVec4>,
            src: Option<&glm::UVec4>)
            -> Result<()>;
}

pub trait Show {
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn show_at<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: &glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}
