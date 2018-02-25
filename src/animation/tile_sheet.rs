use texture::Texture;
use renderer::{options, Draw, Options, Renderer, Show};
use Result;

use glm;

#[derive(Debug, Clone)]
pub struct TileSheet<T> {
    texture: T,
    tiles: glm::UVec2,
    pub dimensions: glm::UVec2,
}

#[derive(Debug)]
pub struct Tile<'a, T: 'a> {
    pub texture: &'a T,
    pub src: glm::UVec4,
}

// https://github.com/rust-lang/rust/issues/40754
// Generics whose type params do not implement Clone, cannot derive Clone
// Manual implementation of it
impl<'a, T> Clone for Tile<'a, T> {
    fn clone(&self) -> Self {
        Tile {
            texture: self.texture,
            ..*self
        }
    }
}

impl<T: Texture> TileSheet<T> {
    pub fn new(tiles: glm::UVec2, texture: T) -> Self {
        TileSheet {
            dimensions: texture.dims() / tiles,
            texture,
            tiles,
        }
    }
}

impl<T> TileSheet<T> {
    pub fn tile(&self, index: u32) -> Tile<T> {
        let tile_pos = glm::uvec2(index % self.tiles.x, index / self.tiles.x);
        let position = tile_pos * self.dimensions;
        let src = glm::uvec4(position.x, position.y, self.dimensions.x, self.dimensions.y);

        Tile {
            texture: &self.texture,
            src,
        }
    }
}

impl<'a, R: Renderer, T: Draw<R>> Show<R> for Tile<'a, T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(self.texture, options::from(self.src))
    }
}

impl<'a, R: Renderer, T: Draw<R>> Draw<R> for Tile<'a, T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(self.texture, options.from(self.src))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_frame() {
        let texture = MockTexture {
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(1, 1), texture);
        let tile = sheet.tile(0);
        assert_eq!(*tile.texture, texture);
        assert_eq!(tile.src, glm::uvec4(0, 0, 10, 10));
    }

    #[test]
    fn single_row() {
        let texture = MockTexture {
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(10, 1), texture);
        let tile = sheet.tile(4);
        assert_eq!(*tile.texture, texture);
        assert_eq!(tile.src, glm::uvec4(4, 0, 1, 10));
    }

    #[test]
    fn single_column() {
        let texture = MockTexture {
            dims: glm::uvec2(10, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(1, 5), texture);
        let tile = sheet.tile(4);
        assert_eq!(*tile.texture, texture);
        assert_eq!(tile.src, glm::uvec4(0, 8, 10, 2));
    }

    #[test]
    fn mult_frames() {
        let texture = MockTexture {
            dims: glm::uvec2(20, 10),
        };
        let sheet = TileSheet::new(glm::uvec2(4, 2), texture);
        let tile = sheet.tile(5);
        assert_eq!(*tile.texture, texture);
        assert_eq!(tile.src, glm::uvec4(5, 5, 5, 5));
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct MockTexture {
        dims: glm::UVec2,
    }

    impl Texture for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            self.dims
        }
    }
}
