pub mod align;
mod font;
mod resource_manager;
mod sdl2;

pub use self::font::*;
pub use self::resource_manager::{FontManager, ResourceManager, TextureManager};

use errors::*;

use glm;
use sdl2::rect;

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    pub dims: Option<glm::UVec2>,
    pub vertical: align::Alignment<align::Vertical>,
    pub horizontal: align::Alignment<align::Horizontal>,
}

impl Destination {
    pub fn dims(mut self, dims: glm::UVec2) -> Destination {
        self.dims = Some(dims);
        self
    }

    pub fn rect<F: FnOnce() -> glm::UVec2>(&self, op: F) -> glm::IVec4 {
        let dims = glm::to_ivec2(self.dims.unwrap_or_else(op));
        let top = {
            let align::Alignment { align, pos } = self.vertical;
            match align {
                align::Vertical::Top => pos,
                align::Vertical::Middle => pos - dims.y / 2,
                align::Vertical::Bottom => pos - dims.y,
            }
        };
        let left = {
            let align::Alignment { align, pos } = self.horizontal;
            match align {
                align::Horizontal::Left => pos,
                align::Horizontal::Center => pos - dims.x / 2,
                align::Horizontal::Right => pos - dims.x,
            }
        };
        glm::ivec4(left, top, dims.x, dims.y)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub angle: f64,
    pub center: glm::IVec2,
}

#[derive(Debug, Clone, Copy)]
pub enum TextureFlip {
    Horizontal,
    Vertical,
    Both,
}

pub mod options {
    use glm;
    use super::{Destination, Options, Rotation, TextureFlip};

    pub fn none() -> Options {
        Options::default()
    }

    pub fn at<D: Into<Destination>>(dst: D) -> Options {
        Options::default().at(dst)
    }

    pub fn from(src: glm::UVec4) -> Options {
        Options::default().from(src)
    }

    pub fn flip(flip: TextureFlip) -> Options {
        Options::default().flip(flip)
    }

    pub fn rotate(rotation: Rotation) -> Options {
        Options::default().rotate(rotation)
    }
}

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub dst: Option<Destination>,
    pub src: Option<glm::UVec4>,
    pub rotation: Option<Rotation>,
    pub flip: Option<TextureFlip>,
}

impl From<glm::IVec4> for Destination {
    fn from(rect: glm::IVec4) -> Destination {
        let horizontal = align::Alignment {
            pos: rect.x,
            align: align::Horizontal::Left,
        };
        let vertical = align::Alignment {
            pos: rect.y,
            align: align::Vertical::Top,
        };
        Destination {
            horizontal,
            vertical,
            dims: Some(glm::uvec2(rect.z as u32, rect.w as u32)),
        }
    }
}

impl From<glm::IVec2> for Destination {
    fn from(tl: glm::IVec2) -> Destination {
        let horizontal = align::Alignment {
            pos: tl.x,
            align: align::Horizontal::Left,
        };
        let vertical = align::Alignment {
            pos: tl.y,
            align: align::Vertical::Top,
        };
        Destination {
            horizontal,
            vertical,
            dims: None,
        }
    }
}

impl Options {
    pub fn at<D: Into<Destination>>(mut self, dst: D) -> Self {
        self.dst = Some(dst.into());
        self
    }

    pub fn from(mut self, src: glm::UVec4) -> Self {
        self.src = Some(src);
        self
    }

    pub fn flip(mut self, flip: TextureFlip) -> Self {
        self.flip = Some(flip);
        self
    }

    pub fn rotate(mut self, rotation: Rotation) -> Self {
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
    fn set_draw_color(&mut self, color: ColorRGBA);
}

pub trait Renderer<'t> {
    type Texture;

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
