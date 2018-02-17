pub mod align;
pub mod options;
mod destination;

pub use self::destination::{Destination, Position};
pub use self::options::Options;
use Result;

use glm;

use std::rc::Rc;

#[derive(Clone, Copy, Debug)]
pub struct ColorRGBA(pub u8, pub u8, pub u8, pub u8);

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

impl<R: Renderer, T: Draw<R>> Draw<R> for Rc<T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(self.as_ref(), options)
    }
}

impl<R: Renderer, T: Show<R>> Show<R> for Rc<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(self.as_ref())
    }
}

pub trait Draw<R: ?Sized>: Show<R> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()>;
}

pub trait Show<R: ?Sized> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

pub trait Canvas: Renderer {
    fn clear(&mut self);
    fn present(&mut self);
}

pub trait Renderer {
    fn set_draw_color(&mut self, color: ColorRGBA);
    fn fill_rects(&mut self, rects: &[Destination]) -> Result<()>;
    fn draw_rects(&mut self, rects: &[Destination]) -> Result<()>;

    /// Default implemenations for drawing assets
    fn draw<A: Draw<Self>>(&mut self, asset: &A, options: Options) -> Result<()> {
        asset.draw(options, self)
    }

    /// Default implementation for drawing scenes
    fn show<S: Show<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }
}
