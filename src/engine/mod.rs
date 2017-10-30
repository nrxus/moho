pub mod step;

#[derive(Debug)]
pub enum State<S> {
    Quit,
    Running(S),
}

impl<S> State<S> {
    pub fn flat_map<F, T>(self, f: F) -> State<T>
    where
        F: FnOnce(S) -> State<T>,
    {
        match self {
            State::Quit => State::Quit,
            State::Running(s) => f(s),
        }
    }

    pub fn map<F, T>(self, f: F) -> State<T>
    where
        F: FnOnce(S) -> T,
    {
        match self {
            State::Quit => State::Quit,
            State::Running(s) => State::Running(f(s)),
        }
    }
}
