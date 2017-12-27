use errors::*;
use renderer::options::{self, Destination, Options, Position};
use renderer::{Draw, Renderer, Show};
use resource;

use glm;
use std::rc::Rc;

pub type Manager<'l, L> = resource::Manager<'l, String, <L as Loader<'l>>::Texture, L>;

pub struct Image<T> {
    pub texture: Rc<T>,
    pub dst: Destination,
}

impl<T> Image<T> {
    pub fn scale(mut self, scale: u32) -> Self {
        self.dst = self.dst.scale(scale);
        self
    }
}

pub trait Texture: Sized {
    fn dims(&self) -> glm::UVec2;

    /// Default implementation for converting to an image
    fn at(self, position: Position) -> Image<Self> {
        let dst = position.dims(self.dims());
        let texture = Rc::new(self);
        Image { texture, dst }
    }
}

pub trait Loader<'a>
    : resource::Loader<'a, <Self as Loader<'a>>::Texture, Args = str> {
    type Texture;
}

impl<R: Renderer, T: Draw<R>> Show<R> for Image<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(&*self.texture, options::at(self.dst))
    }
}

impl<R: Renderer, T: Draw<R>> Draw<R> for Image<T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(&*self.texture, options.at(self.dst))
    }
}
