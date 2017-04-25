use errors::*;
use renderer::{Drawable, Renderer, Scene, Show};

use glm;

use std::time::Duration;

mod animator;
mod tile_sheet;

pub use self::animator::{Animator, AnimatorData};
pub use self::tile_sheet::{Tile, TileSheet};

#[derive(Clone, Debug)]
pub struct AnimationData<T> {
    data: AnimatorData,
    sheet: TileSheet<T>,
}

impl<T> AnimationData<T> {
    pub fn new(data: AnimatorData, sheet: TileSheet<T>) -> AnimationData<T> {
        AnimationData {
            data: data,
            sheet: sheet,
        }
    }

    pub fn start(self) -> Animation<T> {
        Animation::new(self.data.start(), self.sheet)
    }
}

#[derive(Debug)]
pub struct Animation<T> {
    animator: Animator,
    sheet: TileSheet<T>,
}

impl<T> Animation<T> {
    pub fn new(animator: Animator, sheet: TileSheet<T>) -> Animation<T> {
        Animation {
            animator: animator,
            sheet: sheet,
        }
    }

    pub fn animate(&mut self, delta: Duration) -> Tile<T> {
        let frame = self.animator.animate(delta);
        self.sheet.tile(frame)
    }

    pub fn tile(&self) -> Tile<T> {
        let frame = self.animator.frame();
        self.sheet.tile(frame)
    }
}

impl<T, R: Renderer<Texture = T> + Show> Scene<R> for Animation<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.tile())
    }
}

impl<T, R: Renderer<Texture = T> + Show> Drawable<R> for Animation<T> {
    fn draw(&self, dst_rect: &glm::IVec4, renderer: &mut R) -> Result<()> {
        renderer.show_at(&self.tile(), dst_rect)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use renderer::Texture;
    use sdl2::rect;

    #[test]
    fn tile() {
        let texture = Rc::new(MockTexture { dims: glm::uvec2(10, 10) });
        let animator = AnimatorData::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let data = AnimationData::new(animator, sheet);

        let mut animation = data.start();
        assert_eq!(animation.tile().src, glm::uvec4(0, 0, 1, 10));
        let tile = animation.animate(Duration::from_secs(6));
        assert_eq!(tile.src, glm::uvec4(1, 0, 1, 10));
    }

    #[test]
    fn show() {
        let mut renderer = MockRenderer::default();
        let texture = Rc::new(MockTexture { dims: glm::uvec2(10, 10) });
        let animator = AnimatorData::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let data = AnimationData::new(animator, sheet);

        let mut animation = data.start();
        animation.animate(Duration::from_secs(12));
        renderer.show(&animation).unwrap();
        assert_eq!(renderer.last_dst, None);
        assert_eq!(renderer.last_src, Some(glm::uvec4(2, 0, 1, 10)));
        animation.animate(Duration::from_secs(3));
        let dst = glm::ivec4(3, 5, 6, 8);
        renderer.show_at(&animation, &dst).unwrap();
        assert_eq!(renderer.last_dst, Some(dst));
        assert_eq!(renderer.last_src, Some(glm::uvec4(0, 0, 1, 10)));
    }

    struct MockTexture {
        dims: glm::UVec2,
    }

    impl Texture for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            self.dims
        }
    }

    #[derive(Default)]
    struct MockRenderer {
        last_dst: Option<glm::IVec4>,
        last_src: Option<glm::UVec4>,
    }

    impl Renderer for MockRenderer {
        type Texture = MockTexture;

        fn clear(&mut self) {
            unimplemented!();
        }

        fn present(&mut self) {
            unimplemented!();
        }

        fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
            unimplemented!();
        }

        fn copy(&mut self,
                texture: &Self::Texture,
                dst: Option<&glm::IVec4>,
                src: Option<&glm::UVec4>)
                -> Result<()> {
            self.last_dst = dst.cloned();
            self.last_src = src.cloned();
            Ok(())
        }
    }

    impl Show for MockRenderer {}
}
