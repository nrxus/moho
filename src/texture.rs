use renderer::options::{self, Options};
use renderer::{Destination, Draw, Position, Renderer, Show};
use {resource, Result};

use glm;

use std::rc::Rc;

pub type Manager<'l, L> = resource::Manager<'l, String, <L as Loader<'l>>::Texture, L>;

pub struct Image<T> {
    pub texture: T,
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
        Image { texture: self, dst }
    }
}

impl<T: Texture> Texture for Rc<T> {
    fn dims(&self) -> glm::UVec2 {
        self.as_ref().dims()
    }
}

pub trait Loader<'a>: resource::Loader<'a, <Self as Loader<'a>>::Texture, Args = str> {
    type Texture;
}

impl<R: Renderer, T: Draw<R>> Show<R> for Image<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.texture, options::at(self.dst))
    }
}

impl<R: Renderer, T: Draw<R>> Draw<R> for Image<T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(&self.texture, options.at(self.dst))
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use renderer::mocks::MockCanvas;

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct MockTexture {
        pub dims: glm::UVec2,
    }

    impl Texture for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            self.dims
        }
    }

    impl Show<MockCanvas> for MockTexture {
        fn show(&self, _: &mut MockCanvas) -> Result<()> {
            unimplemented!()
        }
    }

    impl Draw<MockCanvas> for MockTexture {
        fn draw(&self, options: Options, renderer: &mut MockCanvas) -> Result<()> {
            renderer.draw.push((*self, options));
            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use renderer::{align, mocks::MockCanvas, options};
    use texture::mocks::MockTexture;

    #[test]
    fn scale_image() {
        let texture = MockTexture {
            dims: glm::uvec2(25, 30),
        };
        let position = align::center(45).top(23);
        let image = texture.at(position).scale(3);
        assert_eq!(image.dst, position.dims(texture.dims * 3));
    }

    #[test]
    fn draws_images() {
        let mut renderer = MockCanvas::new();
        let texture = MockTexture {
            dims: glm::uvec2(25, 30),
        };
        let options = options::from(glm::uvec4(12, 34, 21, 34));
        let position = align::left(45).top(23);
        let image = texture.at(position);
        assert!(renderer.draw(&image, options.clone()).is_ok());
        assert_eq!(renderer.draw.len(), 1);
        let dst = position.dims(texture.dims());
        assert_eq!(renderer.draw[0], (texture, options.at(dst)));
    }

    #[test]
    fn shows_images() {
        let mut renderer = MockCanvas::new();
        let texture = MockTexture {
            dims: glm::uvec2(25, 30),
        };
        let position = align::left(45).top(23);
        let image = texture.at(position);
        assert!(renderer.show(&image).is_ok());
        assert_eq!(renderer.draw.len(), 1);
        let dst = position.dims(texture.dims());
        assert_eq!(renderer.draw[0], (texture, options::at(dst)));
    }
}
