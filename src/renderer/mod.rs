pub mod align;
mod destination;
pub mod options;

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

impl<'a, R: Renderer, T: Show<R>> Show<R> for Vec<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        self.iter().map(|t| renderer.show(t)).collect()
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
    fn draw(&mut self, asset: &impl Draw<Self>, options: Options) -> Result<()> {
        asset.draw(options, self)
    }

    /// Default implementation for drawing scenes
    fn show(&mut self, scene: &impl Show<Self>) -> Result<()> {
        scene.show(self)
    }
}

#[cfg(test)]
pub mod mocks {
    use super::*;
    use texture::mocks::MockTexture;

    #[derive(Default)]
    pub struct MockCanvas {
        pub draw: Vec<(MockTexture, Options)>,
    }

    impl super::Renderer for MockCanvas {
        fn set_draw_color(&mut self, color: ColorRGBA) {}

        fn fill_rects(&mut self, rects: &[Destination]) -> Result<()> {
            Ok(())
        }

        fn draw_rects(&mut self, rects: &[Destination]) -> Result<()> {
            Ok(())
        }
    }

    impl super::Canvas for MockCanvas {
        fn clear(&mut self) {}
        fn present(&mut self) {}
    }

    impl MockCanvas {
        pub fn new() -> Self {
            MockCanvas::default()
        }
    }
}
