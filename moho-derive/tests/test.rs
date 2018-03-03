extern crate glm;
extern crate moho;
#[macro_use]
extern crate moho_derive;

use moho::renderer::{align, ColorRGBA, Destination, Draw, Options, Renderer, Show};
use inner::Assets;
use moho::texture::Texture;

mod inner {
    use moho::texture::Image;

    #[derive(Show)]
    pub struct Assets<T> {
        pub image: Image<T>,
        pub other: T,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MockTexture {
    dims: glm::UVec2,
}

impl Texture for MockTexture {
    fn dims(&self) -> glm::UVec2 {
        self.dims
    }
}

impl Show<MockRenderer> for MockTexture {
    fn show(&self, renderer: &mut MockRenderer) -> moho::Result<()> {
        renderer.shown.push(*self);
        Ok(())
    }
}

impl Draw<MockRenderer> for MockTexture {
    fn draw(&self, options: Options, renderer: &mut MockRenderer) -> moho::Result<()> {
        renderer.drawn.push((*self, options));
        Ok(())
    }
}

#[derive(Default)]
struct MockRenderer {
    drawn: Vec<(MockTexture, Options)>,
    shown: Vec<MockTexture>,
}

impl Renderer for MockRenderer {
    fn set_draw_color(&mut self, _: ColorRGBA) {}

    fn fill_rects(&mut self, _: &[Destination]) -> moho::Result<()> {
        Ok(())
    }

    fn draw_rects(&mut self, _: &[Destination]) -> moho::Result<()> {
        Ok(())
    }
}

#[test]
fn test() {
    use moho::renderer::options;

    let assets = Assets {
        image: MockTexture {
            dims: glm::uvec2(30, 60),
        }.at(align::top(30).left(50)),
        other: MockTexture {
            dims: glm::uvec2(45, 20),
        },
    };

    let mut renderer = MockRenderer::default();
    assert!(renderer.show(&assets).is_ok());
    assert_eq!(renderer.shown, vec![assets.other]);
    assert_eq!(
        renderer.drawn,
        vec![(assets.image.texture, options::at(assets.image.dst))]
    );
}
