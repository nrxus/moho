pub mod sdl2;
pub mod align;
pub mod options;
pub mod font;
mod resource_manager;

pub use self::options::Options;
pub use self::font::{Font, FontLoader};
pub use self::resource_manager::{ResourceManager, TextureManager};

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

pub trait Asset<R: ?Sized>: Scene<R> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R: ?Sized> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait Canvas: Renderer {
    fn clear(&mut self);
    fn present(&mut self);
}

pub trait Renderer {
    fn set_draw_color(&mut self, color: ColorRGBA);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn draw_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;

    /// Default implemenations for drawing assets
    fn draw<A>(&mut self, asset: &A, options: Options) -> Result<()>
    where
        A: Asset<Self>,
    {
        asset.draw(options, self)
    }

    /// Default implementation for drawing scenes
    fn show<S>(&mut self, scene: &S) -> Result<()>
    where
        S: Scene<Self>,
    {
        scene.show(self)
    }
}
