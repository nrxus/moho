use renderer::{Drawable, Scene, Renderer, Texture};
use errors::*;

use glm;

use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct TileSheet<T> {
    texture: Rc<T>,
    tiles: glm::UVec2,
    pub dimensions: glm::UVec2,
}

#[derive(Clone, Debug)]
pub struct Tile<T> {
    pub texture: Rc<T>,
    pub src: glm::UVec4,
}

impl<T: Texture> TileSheet<T> {
    pub fn new(tiles: glm::UVec2, texture: Rc<T>) -> Self {
        let dimensions = texture.dims() / tiles;
        TileSheet {
            texture: texture,
            dimensions: dimensions,
            tiles: tiles,
        }
    }
}

impl<T> TileSheet<T> {
    pub fn tile(&self, index: u32) -> Tile<T> {
        let tile_pos = glm::uvec2(index % self.tiles.x, index / self.tiles.x);
        let position = tile_pos * self.dimensions;
        let src = glm::uvec4(position.x, position.y, self.dimensions.x, self.dimensions.y);

        Tile {
            texture: self.texture.clone(),
            src: src,
        }
    }
}

impl<T, R: Renderer<Texture = T>> Scene<R> for Tile<T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.copy(&*self.texture, None, Some(&self.src))
    }
}

impl<T, R: Renderer<Texture = T>> Drawable<R> for Tile<T> {
    fn draw(&self, dst_rect: &glm::IVec4, renderer: &mut R) -> Result<()> {
        renderer.copy(&*self.texture, Some(dst_rect), Some(&self.src))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_frame() {
        let texture = MockTexture { dims: glm::uvec2(10, 10) };
        let rc_texture = Rc::new(texture);
        let sheet = TileSheet::new(glm::uvec2(1, 1), rc_texture.clone());
        let tile = sheet.tile(0);
        assert_eq!(tile.texture, rc_texture);
        assert_eq!(tile.src, glm::uvec4(0, 0, 10, 10));
    }

    #[test]
    fn single_row() {
        let texture = MockTexture { dims: glm::uvec2(10, 10) };
        let rc_texture = Rc::new(texture);
        let sheet = TileSheet::new(glm::uvec2(10, 1), rc_texture.clone());
        let tile = sheet.tile(4);
        assert_eq!(tile.texture, rc_texture);
        assert_eq!(tile.src, glm::uvec4(4, 0, 1, 10));
    }

    #[test]
    fn single_column() {
        let texture = MockTexture { dims: glm::uvec2(10, 10) };
        let rc_texture = Rc::new(texture);
        let sheet = TileSheet::new(glm::uvec2(1, 5), rc_texture.clone());
        let tile = sheet.tile(4);
        assert_eq!(tile.texture, rc_texture);
        assert_eq!(tile.src, glm::uvec4(0, 8, 10, 2));
    }

    #[test]
    fn mult_frames() {
        let texture = MockTexture { dims: glm::uvec2(20, 10) };
        let rc_texture = Rc::new(texture);
        let sheet = TileSheet::new(glm::uvec2(4, 2), rc_texture.clone());
        let tile = sheet.tile(5);
        assert_eq!(tile.texture, rc_texture);
        assert_eq!(tile.src, glm::uvec4(5, 5, 5, 5));
    }

    #[derive(Debug, PartialEq)]
    struct MockTexture {
        dims: glm::UVec2,
    }

    impl Texture for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            self.dims
        }
    }
}