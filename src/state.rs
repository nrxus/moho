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
    pub fn flat_map<T>(self, f: impl FnOnce(S) -> State<T, Q>) -> State<T, Q> {
        match self {
            State::Quit(q) => State::Quit(q),
            State::Running(s) => f(s),
        }
    }

    pub fn map<T>(self, f: impl FnOnce(S) -> T) -> State<T, Q> {
        match self {
            State::Quit(q) => State::Quit(q),
            State::Running(s) => State::Running(f(s)),
        }
    }

    pub fn map_quit<T>(self, f: impl FnOnce(Q) -> T) -> State<S, T> {
        match self {
            State::Quit(q) => State::Quit(f(q)),
            State::Running(s) => State::Running(s),
        }
    }

    pub fn flat_map_quit<T>(self, f: impl FnOnce(Q) -> State<S, T>) -> State<S, T> {
        match self {
            State::Quit(q) => f(q),
            State::Running(s) => State::Running(s),
        }
    }

    pub fn catch_quit(self, f: impl FnOnce(Q) -> S) -> S {
        match self {
            State::Quit(q) => f(q),
            State::Running(s) => s,
        }
    }
}

impl<S, Q, E> State<Result<S, E>, Q> {
    pub fn transpose(self) -> Result<State<S, Q>, E> {
        match self {
            State::Quit(q) => Ok(State::Quit(q)),
            State::Running(Err(e)) => Err(e),
            State::Running(Ok(s)) => Ok(State::Running(s)),
        }
    }
}
