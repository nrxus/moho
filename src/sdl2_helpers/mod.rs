mod texture;
pub mod font;

use {renderer, Result};

use failure;
use glm;
use sdl2::{pixels, rect, render};
use sdl2::render::RenderTarget;

impl<T: RenderTarget> renderer::Canvas for render::Canvas<T> {
    fn clear(&mut self) {
        self.clear();
    }

    fn present(&mut self) {
        self.present();
    }
}

impl<T: RenderTarget> renderer::Renderer for render::Canvas<T> {
    fn set_draw_color(&mut self, color: renderer::ColorRGBA) {
        let renderer::ColorRGBA(r, g, b, a) = color;
        let color = pixels::Color::RGBA(r, g, b, a);
        self.set_draw_color(color)
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.fill_rects(rects).map_err(failure::err_msg)
    }

    fn draw_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        self.draw_rects(rects).map_err(failure::err_msg)
    }
}

impl<T: RenderTarget> renderer::Window for render::Canvas<T> {
    fn output_size(&self) -> Result<glm::UVec2> {
        let (width, height) = self.output_size().map_err(failure::err_msg)?;
        Ok(glm::uvec2(width, height))
    }
}
