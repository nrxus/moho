use super::{animator, Animation, LimitRun, TileSheet};

#[derive(Debug, Clone)]
pub struct Data<T> {
    pub animator: animator::Data,
    pub sheet: TileSheet<T>,
}

impl<T> Data<T> {
    pub fn start(self) -> Animation<T> {
        Animation::new(self.animator, self.sheet)
    }

    pub fn limit_run_start(self, loops: u32) -> LimitRun<T> {
        LimitRun::new(self.animator, self.sheet, loops)
    }
}
