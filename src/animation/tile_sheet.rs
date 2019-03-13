use crate::{
    renderer::{options, Draw, Options, Renderer, Show},
    texture::Texture,
    Result,
};

#[derive(Debug, Clone)]
pub struct TileSheet<T> {
    texture: T,
    tiles: glm::UVec2,
    dimensions: glm::UVec2,
}

#[derive(Debug)]
pub struct Tile<'a, T> {
    texture: &'a T,
    src: glm::UVec4,
}

// https://github.com/rust-lang/rust/issues/40754
// Generics whose type params do not implement Clone, cannot derive Clone
// Manual implementation of it
impl<T> Clone for Tile<'_, T> {
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
    pub fn tile(&self, index: u32) -> Tile<'_, T> {
        let tile_pos = glm::uvec2(index % self.tiles.x, index / self.tiles.x);
        let position = tile_pos * self.dimensions;
        let src = glm::uvec4(position.x, position.y, self.dimensions.x, self.dimensions.y);

        Tile {
            texture: &self.texture,
            src,
        }
    }
}

impl<T> Tile<'_, T> {
    pub fn rect(&self) -> glm::UVec4 {
        self.src
    }
}

impl<R: Renderer, T: Draw<R>> Show<R> for Tile<'_, T> {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.draw(self.texture, options::from(self.src))
    }
}

impl<R: Renderer, T: Draw<R>> Draw<R> for Tile<'_, T> {
    fn draw(&self, options: Options, renderer: &mut R) -> Result<()> {
        renderer.draw(self.texture, options.from(self.src))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        renderer::{mocks::MockCanvas, options::Flip},
        texture::mocks::MockTexture,
    };

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

    #[test]
    fn draws_tiles() {
        let src = glm::uvec4(5, 5, 5, 5);
        let texture = MockTexture {
            dims: glm::uvec2(20, 10),
        };

        let tile = Tile {
            texture: &texture,
            src,
        };

        let mut renderer = MockCanvas::new();
        let options = options::flip(Flip::Horizontal);
        assert!(renderer.draw(&tile, options.clone()).is_ok());
        assert_eq!(renderer.draw.len(), 1);
        assert_eq!(renderer.draw[0], (texture, options.from(src)));
    }

    #[test]
    fn show_tiles() {
        let src = glm::uvec4(3, 2, 4, 5);
        let texture = MockTexture {
            dims: glm::uvec2(20, 10),
        };

        let tile = Tile {
            texture: &texture,
            src,
        };

        let mut renderer = MockCanvas::new();
        assert!(renderer.show(&tile).is_ok());
        assert_eq!(renderer.draw.len(), 1);
        assert_eq!(renderer.draw[0], (texture, options::from(src)));
    }
}
