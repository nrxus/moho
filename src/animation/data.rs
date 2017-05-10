use super::animator;
use super::{Animation, LimitRun, TileSheet};

#[derive(Clone, Debug)]
pub struct Data<T> {
    pub animator: animator::Data,
    pub sheet: TileSheet<T>,
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
