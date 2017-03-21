extern crate moho;
extern crate sdl2;
extern crate glm;

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;
use sdl2::rect;
use moho::errors::*;
use moho::renderer::*;
use moho::resource_manager::*;
use moho::resource_loader::*;

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

struct RendererTracker {
    load_count: u16,
    last_src: Option<rect::Rect>,
    last_dst: Option<rect::Rect>,
}

impl RendererTracker {
    fn new() -> Self {
        RendererTracker {
            load_count: 0,
            last_dst: None,
            last_src: None,
        }
    }
}

struct MockRenderer {
    error: Option<String>,
    tracker: Rc<RefCell<RendererTracker>>,
}

impl Renderer for MockRenderer {
    type Texture = MockTexture;

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

    fn copy(&mut self,
            texture: &MockTexture,
            src: Option<rect::Rect>,
            dst: Option<rect::Rect>)
            -> Result<()> {
        match self.error {
            None => {
                let mut tracker = self.tracker.borrow_mut();
                tracker.last_src = src;
                tracker.last_dst = dst;
                Ok(())
            }
            Some(ref e) => Err(e.clone().into()),
        }
    }

    fn clear(&mut self) {}

    fn present(&mut self) {}

    fn output_size(&self) -> Result<(u32, u32)> {
        Ok((0, 0))
    }

    fn fill_rects(&mut self, rects: &[rect::Rect]) -> Result<()> {
        Ok(())
    }
}

fn new_subject(error: Option<String>)
               -> (ResourceManager<MockRenderer>, Rc<RefCell<RendererTracker>>) {
    let tracker = Rc::new(RefCell::new(RendererTracker::new()));
    let renderer = MockRenderer {
        error: error,
        tracker: tracker.clone(),
    };

    let subject = ResourceManager::new(renderer);
    (subject, tracker)
}
