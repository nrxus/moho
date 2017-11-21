pub enum Never {}

#[derive(Debug)]
pub enum State<S, Q> {
    Running(S),
    Quit(Q),
}

pub type RunState<S> = State<S, Never>;

impl<S> State<S, Never> {
    pub fn get(self) -> S {
        match self {
            State::Running(s) => s,
            State::Quit(_) => unreachable!(),
        }
    }
}

impl<S, Q> State<S, Q> {
    pub fn flat_map<F, T>(self, f: F) -> State<T, Q>
    where
        F: FnOnce(S) -> State<T, Q>,
    {
        match self {
            State::Quit(q) => State::Quit(q),
            State::Running(s) => f(s),
        }
    }

    pub fn map<F, T>(self, f: F) -> State<T, Q>
    where
        F: FnOnce(S) -> T,
    {
        match self {
            State::Quit(q) => State::Quit(q),
            State::Running(s) => State::Running(f(s)),
        }
    }

    pub fn map_quit<F, T>(self, f: F) -> State<S, T>
    where
        F: FnOnce(Q) -> T,
    {
        match self {
            State::Quit(q) => State::Quit(f(q)),
            State::Running(s) => State::Running(s),
        }
    }

    pub fn flat_map_quit<F, T>(self, f: F) -> State<S, T>
    where
        F: FnOnce(Q) -> State<S, T>,
    {
        match self {
            State::Quit(q) => f(q),
            State::Running(s) => State::Running(s),
        }
    }
}
