use super::options::{Destination, Position};
use super::ResourceManager;

use glm;
use std::rc::Rc;

pub type Manager<'l, L: Loader<'l>> = ResourceManager<'l, String, L::Texture, L>;

pub struct Image<T> {
    pub texture: Rc<T>,
    pub dst: Destination,
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
    : super::Loader<'a, <Self as Loader<'a>>::Texture, Args = str> {
    type Texture;
}
