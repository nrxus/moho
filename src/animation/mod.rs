use std::time::Duration;

mod data;
mod tile_sheet;

pub mod animator;
pub use self::data::Data;
pub use self::animator::Animator;
pub use self::tile_sheet::{Tile, TileSheet};

#[derive(Debug)]
pub struct Animation<T> {
    pub animator: Animator,
    pub sheet: TileSheet<T>,
}

impl<T> Animation<T> {
    pub fn from_data(data: Data<T>) -> Self {
        Self::new(data.animator.start(), data.sheet)
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

#[derive(Debug)]
pub struct LimitRun<T> {
    pub animator: animator::LimitRun,
    pub sheet: TileSheet<T>,
}

impl<T> LimitRun<T> {
    pub fn from_data(data: Data<T>, loops: u32) -> Self {
        Self::new(data.animator.limit_run_start(loops), data.sheet)
    }

    pub fn new(animator: animator::LimitRun, sheet: TileSheet<T>) -> Self {
        LimitRun {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;
    use texture::Texture;
    use glm;

    #[test]
    fn tile() {
        let texture = Rc::new(MockTexture {
            dims: glm::uvec2(10, 10),
        });
        let animator = animator::Data::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let data = Data::new(animator, sheet);

        let mut animation = data.start();
        assert_eq!(animation.tile().src, glm::uvec4(0, 0, 1, 10));
        let tile = animation.animate(Duration::from_secs(6));
        assert_eq!(tile.src, glm::uvec4(1, 0, 1, 10));
    }

    #[test]
    fn limit_run_tile() {
        let texture = Rc::new(MockTexture {
            dims: glm::uvec2(10, 10),
        });
        let animator = animator::Data::new(3, Duration::from_secs(5));
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let data = Data::new(animator, sheet);

        let mut animation = data.limit_run_start(1);
        assert!(animation.tile().is_some());
        assert_eq!(animation.tile().unwrap().src, glm::uvec4(0, 0, 1, 10));

        let tile = animation.animate(Duration::from_secs(6));
        assert!(tile.is_some());
        assert_eq!(tile.unwrap().src, glm::uvec4(1, 0, 1, 10));

        let no_tile = animation.animate(Duration::from_secs(10));
        assert!(no_tile.is_none());
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
}
