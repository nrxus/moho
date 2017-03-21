use errors::*;
use resource_manager::{ResourceManager, Texture, TextureId};
use renderer::{Renderer, ImageDims};

use std::collections::HashMap;
use std::cell::{Ref, RefMut};
use std::path::Path;

pub trait ResourceLoader {
    fn load_texture(&self, path: &'static str) -> Result<Texture>;
}

impl<R: Renderer> ResourceLoader for ResourceManager<R> {
    fn load_texture(&self, path: &'static str) -> Result<Texture> {
        load_cached_texture(self.texture_cache.borrow(), path)
            .map_or_else(|| load_new_texture(self.texture_cache.borrow_mut(),
                                             self.data_cache.borrow_mut(), path, &self.renderer),Ok)
    }
}

fn load_cached_texture(cache: Ref<HashMap<&'static str, Texture>>,
                       path: &'static str)
                       -> Option<Texture> {
    cache.get(path).cloned()
}

fn load_new_texture<R: Renderer>(mut cache: RefMut<HashMap<&'static str, Texture>>,
                                 mut data_cache: RefMut<HashMap<TextureId, R::Texture>>,
                                 path: &'static str,
                                 renderer: &R)
                                 -> Result<Texture> {
    let id = TextureId(data_cache.len());
    let texture_path = Path::new(path);
    let texture_data = renderer.load_texture(texture_path)?;
    let texture = Texture {
        id: id,
        dims: texture_data.dims(),
    };
    cache.insert(path, texture);
    data_cache.insert(id, texture_data);
    Ok(texture)
}
