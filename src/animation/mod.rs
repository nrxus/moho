use errors::*;
use renderer::{Drawable, Renderer, Scene, Show};

use glm;

use std::time::Duration;

mod animator;
mod tile_sheet;

pub use self::animator::{Animator, AnimatorData, LimitRunAnimator};
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
        Animation::from_data(self)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRunAnimation<T> {
        LimitRunAnimation::from_data(self, loops)
    }
}

#[derive(Debug)]
pub struct Animation<T> {
    animator: Animator,
    sheet: TileSheet<T>,
}

impl<T> Animation<T> {
    pub fn from_data(data: AnimationData<T>) -> Self {
        Self::new(data.data.start(), data.sheet)
    }

    pub fn new(animator: Animator, sheet: TileSheet<T>) -> Self {
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

impl<'t, T, R: Renderer<'t, Texture = T> + Show> Scene<R> for Animation<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.show(&self.tile())
    }
}

impl<'t, T, R: Renderer<'t, Texture = T> + Show> Drawable<R> for Animation<T> {
    fn draw(&self, dst_rect: &glm::IVec4, renderer: &mut R) -> Result<()> {
        renderer.show_at(&self.tile(), dst_rect)
    }
}

#[derive(Debug)]
pub struct LimitRunAnimation<T> {
    animator: LimitRunAnimator,
    sheet: TileSheet<T>,
}

impl<T> LimitRunAnimation<T> {
    pub fn from_data(data: AnimationData<T>, loops: u32) -> Self {
        Self::new(data.data.limit_run_start(loops), data.sheet)
    }

    pub fn new(animator: LimitRunAnimator, sheet: TileSheet<T>) -> Self {
        LimitRunAnimation {
            animator: animator,
            sheet: sheet,
        }
    }

    pub fn animate(&mut self, delta: Duration) -> Option<Tile<T>> {
        let frame = self.animator.animate(delta);
        frame.map(|i| self.sheet.tile(i))
    }

    pub fn tile(&self) -> Option<Tile<T>> {
        self.animator.frame().map(|i| self.sheet.tile(i))
    }
}

impl<'t, T, R: Renderer<'t, Texture = T> + Show> Scene<R> for LimitRunAnimation<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        match self.tile() {
            Some(ref t) => renderer.show(t),
            None => Ok(()),
        }
    }
}

impl<'t, T, R: Renderer<'t, Texture = T> + Show> Drawable<R> for LimitRunAnimation<T> {
    fn draw(&self, dst_rect: &glm::IVec4, renderer: &mut R) -> Result<()> {
        match self.tile() {
            Some(ref t) => renderer.show_at(t, dst_rect),
            None => Ok(()),
        }
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
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture.clone());
        let data = AnimationData::new(animator, sheet);

        let mut animation = data.start();
        animation.animate(Duration::from_secs(12));
        renderer.show(&animation).unwrap();
        assert_eq!(renderer.calls.len(), 1);
        {
            let ref call = renderer.calls[0];
            assert_eq!(call.dst, None);
            assert_eq!(call.src, Some(glm::uvec4(2, 0, 1, 10)));
            assert_eq!(call.texture, texture.as_ref() as *const MockTexture);
        }
        animation.animate(Duration::from_secs(3));
        let dst = glm::ivec4(3, 5, 6, 8);
        renderer.show_at(&animation, &dst).unwrap();
        assert_eq!(renderer.calls.len(), 2);
        {
            let ref call = renderer.calls[1];
            assert_eq!(call.dst, Some(dst));
            assert_eq!(call.src, Some(glm::uvec4(0, 0, 1, 10)));
            assert_eq!(call.texture, texture.as_ref() as *const MockTexture);
        }
    }

    #[test]
    fn limit_run_tile() {
        let texture = Rc::new(MockTexture { dims: glm::uvec2(10, 10) });
        let animator = AnimatorData::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let data = AnimationData::new(animator, sheet);

        let mut animation = data.limit_run_start(1);
        assert!(animation.tile().is_some());
        assert_eq!(animation.tile().unwrap().src, glm::uvec4(0, 0, 1, 10));

        let tile = animation.animate(Duration::from_secs(6));
        assert!(tile.is_some());
        assert_eq!(tile.unwrap().src, glm::uvec4(1, 0, 1, 10));

        let no_tile = animation.animate(Duration::from_secs(10));
        assert!(no_tile.is_none());
    }

    #[test]
    fn limit_run_show() {
        let mut renderer = MockRenderer::default();
        let texture = Rc::new(MockTexture { dims: glm::uvec2(10, 10) });
        let animator = AnimatorData::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture.clone());
        let data = AnimationData::new(animator, sheet);

        let mut animation = data.limit_run_start(2);
        animation.animate(Duration::from_secs(12));
        renderer.show(&animation).unwrap();
        assert_eq!(renderer.calls.len(), 1);
        {
            let ref call = renderer.calls[0];
            assert_eq!(call.dst, None);
            assert_eq!(call.src, Some(glm::uvec4(2, 0, 1, 10)));
            assert_eq!(call.texture, texture.as_ref() as *const MockTexture);
        }

        animation.animate(Duration::from_secs(3));
        let dst = glm::ivec4(3, 5, 6, 8);
        renderer.show_at(&animation, &dst).unwrap();
        assert_eq!(renderer.calls.len(), 2);
        {
            let ref call = renderer.calls[1];
            assert_eq!(call.dst, Some(dst));
            assert_eq!(call.src, Some(glm::uvec4(0, 0, 1, 10)));
            assert_eq!(call.texture, texture.as_ref() as *const MockTexture);
        }

        animation.animate(Duration::from_secs(15));
        renderer.show(&animation).unwrap();
        assert_eq!(renderer.calls.len(), 2);
        renderer.show_at(&animation, &dst).unwrap();
        assert_eq!(renderer.calls.len(), 2);
    }

    #[derive(Debug)]
    struct MockTexture {
        dims: glm::UVec2,
    }

    impl Texture for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            self.dims
        }
    }

    #[derive(Debug)]
    struct RenderingCall {
        dst: Option<glm::IVec4>,
        src: Option<glm::UVec4>,
        texture: *const MockTexture,
    }

    #[derive(Default)]
    struct MockRenderer {
        calls: Vec<RenderingCall>,
    }

    impl<'l> Renderer<'l> for MockRenderer {
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
            let call = RenderingCall {
                dst: dst.cloned(),
                src: src.cloned(),
                texture: texture as *const Self::Texture,
            };
            self.calls.push(call);
            Ok(())
        }
    }

    impl Show for MockRenderer {}
}
