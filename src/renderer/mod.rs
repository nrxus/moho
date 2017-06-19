mod font;
mod resource_manager;
mod sdl2;

pub use self::font::*;
pub use self::resource_manager::{FontManager, ResourceManager, TextureManager};

use errors::*;

use glm;
use sdl2::rect;

pub struct Rotation {
    pub angle: f64,
    pub center: glm::IVec2,
}

pub enum TextureFlip {
    Horizontal,
    Vertical,
    Both,
}

pub mod options {
    use glm;
    use super::Options;
    use super::TextureFlip;
    use super::Rotation;

    pub fn none<'a>() -> Options<'a> {
        Options::default()
    }

    pub fn at<'a>(dst: &'a glm::IVec4) -> Options<'a> {
        Options::default().at(dst)
    }

    pub fn from<'a>(src: &'a glm::UVec4) -> Options<'a> {
        Options::default().from(src)
    }

    pub fn flip<'a>(flip: TextureFlip) -> Options<'a> {
        Options::default().flip(flip)
    }

    pub fn rotate<'a>(rotation: &'a Rotation) -> Options<'a> {
        Options::default().rotate(rotation)
    }
}

#[derive(Default)]
pub struct Options<'a> {
    pub dst: Option<&'a glm::IVec4>,
    pub src: Option<&'a glm::UVec4>,
    pub rotation: Option<&'a Rotation>,
    pub flip: Option<TextureFlip>,
}

impl<'a> Options<'a> {
    pub fn at(mut self, dst: &'a glm::IVec4) -> Self {
        self.dst = Some(dst);
        self
    }

    pub fn from(mut self, src: &'a glm::UVec4) -> Self {
        self.src = Some(src);
        self
    }

    pub fn flip(mut self, flip: TextureFlip) -> Self {
        self.flip = Some(flip);
        self
    }

    pub fn rotate(mut self, rotation: &'a Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

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
    type Texture: Texture;
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
    fn set_draw_color(&mut self, color: ColorRGBA);
}

pub trait Renderer<'t> {
    type Texture;

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
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
