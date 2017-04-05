mod font;

use errors::*;
use renderer;

use glm;
use sdl2::image::LoadTexture;
use sdl2::rect;
use sdl2::render;

impl renderer::Texture for render::Texture {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

impl renderer::ResourceLoader for render::Renderer<'static> {
    type Texture = render::Texture;
}

impl<'a> renderer::Loader<'a, render::Texture, str> for render::Renderer<'static> {
    fn load(&'a self, path: &str) -> Result<render::Texture> {
        self.load_texture(path).map_err(Into::into)
    }
}

use sdl2::render::Texture as SdlTexture;

impl renderer::Renderer for render::Renderer<'static> {
    type Texture = SdlTexture;
    fn copy(&mut self,
            texture: &SdlTexture,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()> {
        let src = src.map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = dst.map(|r| rect::Rect::new(r.x, r.y, r.z as u32, r.w as u32));
        self.copy(texture, src, dst).map_err(Into::into)
    }

    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.fill_rects(rects).map_err(Into::into)
    }
}

impl renderer::Show for render::Renderer<'static> {}

impl renderer::Window for render::Renderer<'static> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size()?;
        Ok(glm::uvec2(width, height))
    }
}
