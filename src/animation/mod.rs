mod data;
mod tile_sheet;

pub mod animator;

pub use self::{
    animator::Animator,
    data::Data,
    tile_sheet::{Tile, TileSheet},
};

use std::time::Duration;

#[derive(Debug)]
pub struct Animation<T> {
    animator: Animator,
    sheet: TileSheet<T>,
}

impl<T> Animation<T> {
    pub fn new(animator: animator::Data, sheet: TileSheet<T>) -> Self {
        Animation {
            animator: animator.start(),
            sheet,
        }
    }

    pub fn animate(&mut self, delta: Duration) -> Tile<'_, T> {
        let frame = self.animator.animate(delta);
        self.sheet.tile(frame)
    }

    pub fn tile(&self) -> Tile<'_, T> {
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
    pub fn new(animator: animator::Data, sheet: TileSheet<T>, loops: u32) -> Self {
        LimitRun {
            animator: animator.limit_run_start(loops),
            sheet,
        }
    }

    pub fn animate(&mut self, delta: Duration) -> Option<Tile<'_, T>> {
        let frame = self.animator.animate(delta);
        let sheet = &self.sheet;
        frame.map(|i| sheet.tile(i))
    }

    pub fn tile(&self) -> Option<Tile<'_, T>> {
        self.animator.frame().map(|i| self.sheet.tile(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::texture::mocks::MockTexture;

    #[test]
    fn tile() {
        let texture = MockTexture {
            dims: glm::uvec2(10, 10),
        };
        let animator = animator::Data {
            max: 3,
            duration: Duration::from_secs(5),
        };
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);

        let mut animation = Animation::new(animator, sheet);
        assert_eq!(animation.tile().rect(), glm::uvec4(0, 0, 1, 10));
        let tile = animation.animate(Duration::from_secs(6));
        assert_eq!(tile.rect(), glm::uvec4(1, 0, 1, 10));
    }

    #[test]
    fn limit_run_tile() -> Result<(), String> {
        let texture = MockTexture {
            dims: glm::uvec2(10, 10),
        };
        let animator = animator::Data {
            max: 3,
            duration: Duration::from_secs(5),
        };
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);

        let mut animation = LimitRun::new(animator, sheet, 1);
        let tile = animation.tile().ok_or("animation ended unexpectedly")?;
        assert_eq!(tile.rect(), glm::uvec4(0, 0, 1, 10));

        let tile = animation
            .animate(Duration::from_secs(6))
            .ok_or("animation ended unexpectedly")?;
        assert_eq!(tile.rect(), glm::uvec4(1, 0, 1, 10));

        let no_tile = animation.animate(Duration::from_secs(10));
        assert!(no_tile.is_none());
        Ok(())
    }
}
