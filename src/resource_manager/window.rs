use super::ResourceManager;
use errors::*;

use glm;
use sdl2::render::Renderer as SdlRenderer;

pub trait BackEndWindow: super::BackEnd {
    fn output_size(&self) -> Result<(u32, u32)>;
}

impl BackEndWindow for SdlRenderer<'static> {
    fn output_size(&self) -> Result<(u32, u32)> {
        self.output_size().map_err(Into::into)
    }
}

pub trait Window {
    fn output_size(&self) -> Result<glm::UVec2>;
}

impl<R: BackEndWindow> Window for ResourceManager<R> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (x, y) = self.renderer.output_size()?;
        Ok(glm::uvec2(x, y))
    }
}
