use super::animator;
use super::{Animation, LimitRun, TileSheet};

#[derive(Debug)]
pub struct Data<T> {
    pub animator: animator::Data,
    pub sheet: TileSheet<T>,
}

// https://github.com/rust-lang/rust/issues/40754
// Generics whose type params do not implement Clone, cannot derive Clone
// Manual implementation of it
impl<T> Clone for Data<T> {
    fn clone(&self) -> Data<T> {
        Data {
            animator: self.animator,
            sheet: self.sheet.clone(),
        }
    }
}

impl<T> Data<T> {
    pub fn new(data: animator::Data, sheet: TileSheet<T>) -> Data<T> {
        Data {
            animator: data,
            sheet: sheet,
        }
    }

    pub fn start(self) -> Animation<T> {
        Animation::from_data(self)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRun<T> {
        LimitRun::from_data(self, loops)
    }
}
