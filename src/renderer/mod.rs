pub mod align;
pub mod options;
mod font;
mod resource_manager;
mod sdl2;

pub use self::options::Options;
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

pub trait TextureLoader<'a>
    : Loader<'a, <Self as TextureLoader<'a>>::Texture, Args = str> {
    type Texture;
}

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

pub trait Asset<R: ?Sized> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: ?Sized> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait Canvas<'t>: Renderer<'t> {
    fn clear(&mut self);
    fn present(&mut self);
}

pub trait Renderer<'t> {
    type Texture;

    fn set_draw_color(&mut self, color: ColorRGBA);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn draw_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self, texture: &Self::Texture, options: Options) -> Result<()>;

    /// Default implemenations for drawing assets
    fn copy_asset<A>(&mut self, drawable: &A, options: Options) -> Result<()>
    where
        A: Asset<Self>,
    {
        drawable.draw(options, self)
    }

    /// Default implementation for drawing scenes
    fn show<S>(&mut self, scene: &S) -> Result<()>
    where
        S: Scene<Self>,
    {
        scene.show(self)
    }
}
