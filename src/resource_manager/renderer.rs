use super::{Drawable, Scene};
use errors::*;

use glm;
use sdl2::pixels::Color;
use sdl2::rect;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;
use sdl2::ttf::Font;

pub trait Renderer: super::BackEnd + Sized {
    fn clear(&mut self);
    fn present(&mut self);
    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()>;
    fn copy(&mut self,
            texture: &Self::Texture,
            dst: Option<glm::IVec4>,
            src: Option<glm::UVec4>)
            -> Result<()>;
    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()>;
    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()>;
}

pub trait FontTexturizer: super::BackEnd {
    fn texturize(&self, font: &Font, text: &str) -> Result<Self::Texture>;
}

impl FontTexturizer for SdlRenderer<'static> {
    fn texturize(&self, font: &Font, text: &str) -> Result<SdlTexture> {
        let surface = font.render(text)
            .blended(Color::RGBA(255, 0, 0, 255))
            .chain_err(|| "error when creatinga a blended font surface")?;

        self.create_texture_from_surface(&surface)
            .chain_err(|| "error creating a texture from a surface")
    }
}

impl Renderer for SdlRenderer<'static> {
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

    fn show<S: Scene<Self>>(&mut self, scene: &S) -> Result<()> {
        scene.show(self)
    }

    fn render<D: Drawable<Self>>(&mut self, drawable: &D, dst_rect: glm::IVec4) -> Result<()> {
        drawable.draw(dst_rect, self)
    }
}
