use crate::Result;

use std::{borrow::Borrow, collections::HashMap, hash::Hash, rc::Rc};

pub trait Loader<'a, T> {
    type Args: ?Sized;

    fn load(&'a self, data: &Self::Args) -> Result<T>;
}

pub struct Manager<'l, K, R, L>
where
    K: Hash + Eq,
{
    loader: &'l L,
    pub cache: HashMap<K, Rc<R>>,
}

impl<'l, K, R, L> Manager<'l, K, R, L>
where
    K: Hash + Eq,
{
    pub fn new(loader: &'l L) -> Self {
        Manager {
            cache: HashMap::new(),
            loader,
        }
    }

    pub fn load<D>(&mut self, details: &D) -> Result<Rc<R>>
    where
        K: Borrow<D> + for<'b> From<&'b D>,
        L: Loader<'l, R, Args = D>,
        D: Eq + Hash + ?Sized,
    {
        self.cache.get(details).cloned().map_or_else(
            || {
                let resource = self.loader.load(details).map(Rc::new)?;
                self.cache.insert(details.into(), Rc::clone(&resource));
                Ok(resource)
            },
            Ok,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;

    type LoadTracker = Rc<Cell<Counter>>;

    #[test]
    fn loads_resource() {
        let (loader, tracker) = loader(None);
        let mut subject: Manager<String, _, _> = Manager::new(&loader);
        let texture = subject.load("mypath/").unwrap();
        assert_eq!(texture.path, "mypath/");
        assert_eq!(tracker.get(), Counter(1));
    }

    #[test]
    fn returns_error() {
        let (loader, tracker) = loader(Some("FAIL".into()));
        let mut subject: Manager<String, _, _> = Manager::new(&loader);
        let result = subject.load("mypath/");
        assert_eq!(result.is_err(), true);
        assert_eq!(tracker.get(), Counter(1));
    }

    #[test]
    fn caches_resources() {
        let (loader, tracker) = loader(None);
        let mut subject: Manager<String, _, _> = Manager::new(&loader);

        //get new resource - number of calls 1
        let texture = subject.load("mypath/1").unwrap();
        assert_eq!(texture.path, "mypath/1");
        assert_eq!(tracker.get(), Counter(1));

        //get new resource - number of calls 1
        let texture = subject.load("mypath/1").unwrap();
        assert_eq!(texture.path, "mypath/1");
        assert_eq!(tracker.get(), Counter(1));

        //get new resource - number of calls 1
        let texture = subject.load("mypath/2").unwrap();
        assert_eq!(texture.path, "mypath/2");
        assert_eq!(tracker.get(), Counter(2));
    }

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

    impl<'a> Loader<'a, MockResource> for MockLoader {
        type Args = str;

        fn load(&self, data: &str) -> Result<MockResource> {
            let mut counter = self.tracker.get();
            counter.increase();
            self.tracker.set(counter);
            match self.error {
                None => Ok(MockResource { path: data.into() }),
                Some(ref e) => Err(failure::err_msg(e.clone())),
            }
        }
    }

    fn loader(error: Option<String>) -> (MockLoader, LoadTracker) {
        let tracker = Rc::new(Cell::new(Counter(0)));
        let loader = MockLoader {
            error,
            tracker: Rc::clone(&tracker),
        };
        (loader, tracker)
    }
}
