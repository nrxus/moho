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
