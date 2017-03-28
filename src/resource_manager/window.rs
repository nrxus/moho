use errors::*;

use glm;
use sdl2::render::Renderer as SdlRenderer;

pub trait Window: super::BackEnd {
    fn output_size(&self) -> Result<glm::UVec2>;
}

impl Window for SdlRenderer<'static> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size()?;
        Ok(glm::uvec2(width, height))
    }
}
