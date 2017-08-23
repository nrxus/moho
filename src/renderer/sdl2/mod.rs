mod font;

use errors::*;
use renderer::{self, align};

use glm;
use sdl2::image::LoadTexture;
use sdl2::rect;
use sdl2::render::{self, RenderTarget};
use sdl2::pixels;

impl<'c> renderer::Texture for render::Texture<'c> {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

impl<'c, T> renderer::TextureLoader<'c> for render::TextureCreator<T> {
    type Texture = render::Texture<'c>;
}

impl<'c, T> renderer::Loader<'c, render::Texture<'c>> for render::TextureCreator<T> {
    type Args = str;
    fn load(&'c self, path: &str) -> Result<render::Texture<'c>> {
        self.load_texture(path).map_err(Into::into)
    }
}

use sdl2::render::Texture as SdlTexture;

impl<'t, T: RenderTarget> renderer::Canvas<'t> for render::Canvas<T> {
    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }

    fn set_draw_color(&mut self, color: renderer::ColorRGBA) {
        let renderer::ColorRGBA(r, g, b, a) = color;
        let color = pixels::Color::RGBA(r, g, b, a);
        self.set_draw_color(color)
    }
}

impl<'t, T: RenderTarget> renderer::Renderer<'t> for render::Canvas<T> {
    type Texture = SdlTexture<'t>;

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.fill_rects(rects).map_err(Into::into)
    }

    fn draw_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.draw_rects(rects).map_err(Into::into)
    }

    fn copy(&mut self, texture: &Self::Texture, options: renderer::Options) -> Result<()> {
        let src = options
            .src
            .map(|r| rect::Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = options.dst.map(|d| {
            let dims = d.dims.unwrap_or_else(|| {
                let query = texture.query();
                glm::uvec2(query.width, query.height)
            });
            let top = {
                let align::Alignment { align, pos } = d.vertical;
                match align {
                    align::Vertical::Top => pos,
                    align::Vertical::Middle => pos - dims.y as i32 / 2,
                    align::Vertical::Bottom => pos - dims.y as i32,
                }
            };
            let left = {
                let align::Alignment { align, pos } = d.horizontal;
                match align {
                    align::Horizontal::Left => pos,
                    align::Horizontal::Center => pos - dims.x as i32 / 2,
                    align::Horizontal::Right => pos - dims.x as i32,
                }
            };

            rect::Rect::new(left, top, dims.x, dims.y)
        });
        match (options.rotation, options.flip) {
            (None, None) => self.copy(texture, src, dst).map_err(Into::into),
            (r, f) => {
                let (angle, center) = match r {
                    None => (0., None),
                    Some(r) => (r.angle, Some(rect::Point::new(r.center.x, r.center.y))),
                };
                let (hflip, vflip) = match f {
                    None => (false, false),
                    Some(renderer::TextureFlip::Horizontal) => (true, false),
                    Some(renderer::TextureFlip::Vertical) => (false, true),
                    Some(renderer::TextureFlip::Both) => (true, true),
                };
                self.copy_ex(texture, src, dst, angle, center, hflip, vflip)
                    .map_err(Into::into)
            }
        }
    }
}

impl<T: RenderTarget> renderer::Window for render::Canvas<T> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size()?;
        Ok(glm::uvec2(width, height))
    }
}
