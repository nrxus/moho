use super::{BackEnd, ResourceLoader};
use errors::*;

use std::collections::HashMap;
use std::rc::Rc;
use std::path::Path;

pub struct ResourceManager<R: BackEnd> {
    cache: HashMap<String, Rc<R::Texture>>,
}

impl<R: BackEnd> ResourceManager<R> {
    pub fn new() -> Self {
        ResourceManager { cache: HashMap::new() }
    }
}

impl<R: ResourceLoader> ResourceManager<R> {
    pub fn load_texture(&mut self, path: &str, loader: &R) -> Result<Rc<R::Texture>> {
        Self::load_cached_texture(&self.cache, path)
            .map_or_else(|| Self::load_new_texture(&mut self.cache, path, loader),Ok)
    }

    fn load_cached_texture(cache: &HashMap<String, Rc<R::Texture>>,
                           path: &str)
                           -> Option<Rc<R::Texture>> {
        cache.get(path).cloned()
    }

    fn load_new_texture(cache: &mut HashMap<String, Rc<R::Texture>>,
                        path: &str,
                        renderer: &R)
                        -> Result<Rc<R::Texture>> {
        let texture_path = Path::new(path);
        let texture = renderer.load_texture(texture_path)?;
        let texture = Rc::new(texture);
        cache.insert(path.into(), texture.clone());
        Ok(texture)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use resource_manager::ImageDims;
    use glm;
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn loads_texture_data() {
        let (mut subject, mut loader, tracker) = new_subject(None);
        let texture = subject.load_texture("mypath/", &mut loader).unwrap();
        assert_eq!(texture.path, "mypath/");
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn returns_error() {
        let (mut subject, mut loader, tracker) = new_subject(Some("FAIL".into()));
        let texture_data = subject.load_texture("mypath/", &mut loader);
        assert_eq!(texture_data.err().is_some(), true);
        assert_eq!(tracker.borrow().load_count, 1);
    }

    #[test]
    fn caches_texture_datas() {
        let (mut subject, mut loader, tracker) = new_subject(None);

        // get a new texture_data - number of calls is 1
        let texture1 = subject.load_texture("mypath/1", &mut loader).unwrap();
        assert_eq!(texture1.path, "mypath/1");
        assert_eq!(tracker.borrow().load_count, 1);

        // load the same texture_data - number of calls should still be 1
        let texture2 = subject.load_texture("mypath/1", &mut loader).unwrap();
        assert_eq!(texture2.path, "mypath/1");
        assert_eq!(tracker.borrow().load_count, 1);

        // load a different texture_data - number of calls should increase
        let texture3 = subject.load_texture("mypath/2", &mut loader).unwrap();
        assert_eq!(texture3.path, "mypath/2");
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

    impl ResourceLoader for MockBackEnd {
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
                   -> (ResourceManager<MockBackEnd>, MockBackEnd, Rc<RefCell<LoaderTracker>>) {
        let tracker = Rc::new(RefCell::new(LoaderTracker::new()));
        let renderer = MockBackEnd {
            error: error,
            tracker: tracker.clone(),
        };

        let subject = ResourceManager::new();
        (subject, renderer, tracker)
    }
}
