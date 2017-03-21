extern crate moho;
extern crate sdl2;
extern crate glm;

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use moho::errors::*;
use moho::resource_manager::*;

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

impl BackEnd for MockBackEnd {
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
