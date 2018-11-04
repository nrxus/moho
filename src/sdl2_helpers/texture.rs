use crate::{
    renderer::{self, options},
    resource, texture as moho, Result,
};

use sdl2::{
    image::LoadTexture,
    rect::{Point, Rect},
    render::{Canvas, RenderTarget, Texture, TextureCreator},
};

impl moho::Texture for Texture<'_> {
    fn dims(&self) -> glm::UVec2 {
        let query = self.query();
        glm::uvec2(query.width, query.height)
    }
}

impl<'c, T> moho::Loader<'c> for TextureCreator<T> {
    type Texture = Texture<'c>;
}

impl<'c, T> resource::Loader<'c, Texture<'c>> for TextureCreator<T> {
    type Args = str;

    fn load(&'c self, path: &str) -> Result<Texture<'c>> {
        self.load_texture(path).map_err(failure::err_msg)
    }
}

impl<T: RenderTarget> renderer::Show<Canvas<T>> for Texture<'_> {
    fn show(&self, renderer: &mut Canvas<T>) -> Result<()> {
        renderer.copy(self, None, None).map_err(failure::err_msg)
    }
}

impl From<renderer::Destination> for Rect {
    fn from(dst: renderer::Destination) -> Rect {
        Rect::new(dst.left(), dst.top(), dst.dims.x, dst.dims.y)
    }
}

impl<T: RenderTarget> renderer::Draw<Canvas<T>> for Texture<'_> {
    fn draw(&self, options: renderer::Options, renderer: &mut Canvas<T>) -> Result<()> {
        let src = options
            .src
            .map(|r| Rect::new(r.x as i32, r.y as i32, r.z, r.w));
        let dst = options.dst.map(Into::into);
        match (options.rotation, options.flip) {
            (None, None) => renderer.copy(self, src, dst).map_err(failure::err_msg),
            (r, f) => {
                let (angle, center) = match r {
                    None => (0., None),
                    Some(r) => (r.angle, Some(Point::new(r.center.x, r.center.y))),
                };
                let (hflip, vflip) = match f {
                    None => (false, false),
                    Some(options::Flip::Horizontal) => (true, false),
                    Some(options::Flip::Vertical) => (false, true),
                    Some(options::Flip::Both) => (true, true),
                };
                renderer
                    .copy_ex(self, src, dst, angle, center, hflip, vflip)
                    .map_err(failure::err_msg)
            }
        }
    }
}
