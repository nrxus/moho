use renderer::Loader;
use errors::*;

use std::borrow::Borrow;
use std::hash::Hash;
use std::collections::HashMap;
use std::rc::Rc;

pub struct ResourceManager<R, C: Hash + Eq> {
    cache: HashMap<C, Rc<R>>,
}

impl<R, C: Hash + Eq> ResourceManager<R, C> {
    pub fn new() -> Self {
        ResourceManager { cache: HashMap::new() }
    }

    pub fn load<'a, L, D>(&mut self, details: &D, loader: &'a L) -> Result<Rc<R>>
        where L: Loader<'a, R, D>,
              D: Eq + Hash + ?Sized,
              C: Borrow<D> + for<'b> From<&'b D>
    {
        self.cache
            .get(details)
            .cloned()
            .map_or_else(|| {
                             let resource = Rc::new(loader.load(details)?);
                             self.cache.insert(details.into(), resource.clone());
                             Ok(resource)
                         },
                         Ok)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type LoadTracker = Rc<Cell<Counter>>;

    #[test]
    fn loads_resource() {
        let (mut subject, mut loader, tracker) = init(None);
        let texture = subject.load("mypath/", &mut loader).unwrap();
        assert_eq!(texture.path, "mypath/");
        assert_eq!(tracker.get(), Counter(1));
    }

    #[test]
    fn returns_error() {
        let (mut subject, mut loader, tracker) = init(Some("FAIL".into()));
        let result = subject.load("mypath/", &mut loader);
        assert_eq!(result.is_err(), true);
        assert_eq!(tracker.get(), Counter(1));
    }

    #[test]
    fn caches_resources() {
        let (mut subject, mut loader, tracker) = init(None);

        //get new resource - number of calls 1
        let texture = subject.load("mypath/1", &mut loader).unwrap();
        assert_eq!(texture.path, "mypath/1");
        assert_eq!(tracker.get(), Counter(1));

        //get new resource - number of calls 1
        let texture = subject.load("mypath/1", &mut loader).unwrap();
        assert_eq!(texture.path, "mypath/1");
        assert_eq!(tracker.get(), Counter(1));

        //get new resource - number of calls 1
        let texture = subject.load("mypath/2", &mut loader).unwrap();
        assert_eq!(texture.path, "mypath/2");
        assert_eq!(tracker.get(), Counter(2));
    }

    use std::cell::Cell;

    #[derive(Debug)]
    struct MockResource {
        path: String,
    }

    struct MockLoader {
        error: Option<String>,
        tracker: LoadTracker,
    }

    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Counter(u16);

    impl Counter {
        fn increase(&mut self) {
            self.0 += 1;
        }
    }

    impl<'a> Loader<'a, MockResource, str> for MockLoader {
        fn load(&self, data: &str) -> Result<MockResource> {
            let mut counter = self.tracker.get();
            counter.increase();
            self.tracker.set(counter);
            match self.error {
                None => Ok(MockResource { path: data.into() }),
                Some(ref e) => Err(e.clone().into()),
            }
        }
    }

    fn init(error: Option<String>)
            -> (ResourceManager<MockResource, String>, MockLoader, LoadTracker) {
        let tracker = Rc::new(Cell::new(Counter(0)));
        let loader = MockLoader {
            error: error,
            tracker: tracker.clone(),
        };

        let subject = ResourceManager::new();
        (subject, loader, tracker)
    }
}
