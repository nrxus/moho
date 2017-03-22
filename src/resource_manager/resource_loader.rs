use super::{ImageDims, ResourceManager, Texture, TextureId};
use errors::*;

use sdl2::image::LoadTexture;
use sdl2::render::Renderer as SdlRenderer;
use sdl2::render::Texture as SdlTexture;

use std::collections::HashMap;
use std::cell::{Ref, RefMut};
use std::path::Path;

pub trait BackEndLoader: super::BackEnd {
    fn load_texture(&self, path: &Path) -> Result<Self::Texture>;
}

impl BackEndLoader for SdlRenderer<'static> {
    fn load_texture(&self, path: &Path) -> Result<SdlTexture> {
        LoadTexture::load_texture(self, path).map_err(Into::into)
    }
}

pub trait ResourceLoader {
    fn load_texture(&self, path: &'static str) -> Result<Texture>;
}

impl<R: BackEndLoader> ResourceLoader for ResourceManager<R> {
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

fn load_new_texture<R: BackEndLoader>(mut cache: RefMut<HashMap<&'static str, Texture>>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use glm;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn loads_texture_data() {
        let (subject, tracker) = new_subject(None);
        let texture = subject.load_texture("mypath/").unwrap();
        assert_eq!(texture.id, TextureId(0));
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn returns_error() {
        let (subject, tracker) = new_subject(Some("FAIL".into()));
        let texture_data = subject.load_texture("mypath/");
        assert_eq!(texture_data.err().is_some(), true);
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn caches_texture_datas() {
        let (subject, tracker) = new_subject(None);

        // get a new texture_data - number of calls is 1
        let texture1 = subject.load_texture("mypath/1").unwrap();
        assert_eq!(texture1.id, TextureId(0));
        assert_eq!(tracker.borrow().load_count, 1);

        // load the same texture_data - number of calls should still be 1
        let texture2 = subject.load_texture("mypath/1").unwrap();
        assert_eq!(texture2.id, TextureId(0));
        assert_eq!(tracker.borrow().load_count, 1);

        // load a different texture_data - number of calls should increase
        let texture3 = subject.load_texture("mypath/2").unwrap();
        assert_eq!(texture3.id, TextureId(1));
        assert_eq!(tracker.borrow().load_count, 2);
    }

    #[derive(Debug)]
    struct MockTexture {
        path: String,
    }

    impl ImageDims for MockTexture {
        fn dims(&self) -> glm::UVec2 {
            glm::uvec2(50, 50)
        }
    }

    struct LoaderTracker {
        load_count: u16,
    }

    impl LoaderTracker {
        fn new() -> Self {
            LoaderTracker { load_count: 0 }
        }
    }

    struct MockBackEnd {
        error: Option<String>,
        tracker: Rc<RefCell<LoaderTracker>>,
    }

    impl super::super::BackEnd for MockBackEnd {
        type Texture = MockTexture;
    }

    impl BackEndLoader for MockBackEnd {
        fn load_texture(&self, path: &Path) -> Result<MockTexture> {
            self.tracker.borrow_mut().load_count += 1;
            match self.error {
                None => {
                    let texture = MockTexture { path: path.to_str().unwrap_or("").into() };
                    Ok(texture)
                }
                Some(ref e) => Err(e.clone().into()),
            }
        }
    }

    fn new_subject(error: Option<String>)
                   -> (ResourceManager<MockBackEnd>, Rc<RefCell<LoaderTracker>>) {
        let tracker = Rc::new(RefCell::new(LoaderTracker::new()));
        let renderer = MockBackEnd {
            error: error,
            tracker: tracker.clone(),
        };

        let subject = ResourceManager::new(renderer);
        (subject, tracker)
    }
}
