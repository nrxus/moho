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

#[derive(Default)]
pub struct Options<'a> {
    dst: Option<&'a glm::IVec4>,
    src: Option<&'a glm::UVec4>,
    rotation: Option<&'a Rotation>,
    flip: Option<TextureFlip>,
}

impl<'a> Options<'a> {
    pub fn new() -> Self {
        Self::default()
    }

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

pub trait Renderer<'t> {
    type Texture;

    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self, texture: &Self::Texture, options: Options) -> Result<()>;
    fn copy_asset<'a, A>(&'a mut self, drawable: &'a A, options: Options) -> Result<()>
        where A: Asset<Self>
    {
        drawable.draw(options, self)
    }
}

pub trait Show {
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }
}
