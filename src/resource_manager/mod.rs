mod frame_animator;
mod font_manager;
mod window;
mod renderer;
mod resource_loader;
mod tile_sheet;

pub use self::font_manager::FontManager;
pub use self::frame_animator::FrameAnimator;
pub use self::window::Window;
pub use self::renderer::{FontTexturizer, Renderer};
pub use self::resource_loader::{ResourceLoader, ResourceManager};
pub use self::tile_sheet::{Tile, TileSheet};

use errors::*;

use glm;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

pub trait ImageDims {
    fn dims(&self) -> glm::UVec2;
}

impl ImageDims for SdlTexture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

pub trait BackEnd {
    type Texture: ImageDims;
}

impl BackEnd for SdlRenderer<'static> {
    type Texture = SdlTexture;
}

pub trait Drawable<R> {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()>;
}

pub trait Scene<R> {
    fn show(&self, renderer: &mut R) -> Result<()>;
}

impl<R: Renderer<Texture = SdlTexture>> Scene<R> for SdlTexture {
    fn show(&self, renderer: &mut R) -> Result<()> {
        renderer.copy(self, None, None)
    }
}

impl<R: Renderer<Texture = SdlTexture>> Drawable<R> for SdlTexture {
    fn draw(&self, dst_rect: glm::IVec4, renderer: &mut R) -> Result<()> {
        renderer.copy(self, Some(dst_rect), None)
    }
}
