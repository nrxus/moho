use renderer::Position;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vertical {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Alignment<T> {
    pub align: T,
    pub pos: i32,
}

impl<T> Alignment<T> {
    pub fn nudge(mut self, delta: i32) -> Self {
        self.pos += delta;
        self
    }
}

impl Alignment<Vertical> {
    pub fn left(self, pos: i32) -> Position {
        Position {
            horizontal: left(pos),
            vertical: self,
        }
    }

    pub fn center(self, pos: i32) -> Position {
        Position {
            horizontal: center(pos),
            vertical: self,
        }
    }

    pub fn right(self, pos: i32) -> Position {
        Position {
            horizontal: right(pos),
            vertical: self,
        }
    }
}

impl Alignment<Horizontal> {
    pub fn top(self, pos: i32) -> Position {
        Position {
            vertical: top(pos),
            horizontal: self,
        }
    }

    pub fn middle(self, pos: i32) -> Position {
        Position {
            vertical: middle(pos),
            horizontal: self,
        }
    }

    pub fn bottom(self, pos: i32) -> Position {
        Position {
            vertical: bottom(pos),
            horizontal: self,
        }
    }
}

pub fn top(pos: i32) -> Alignment<Vertical> {
    Alignment {
        pos,
        align: Vertical::Top,
    }
}

pub fn middle(pos: i32) -> Alignment<Vertical> {
    Alignment {
        pos,
        align: Vertical::Middle,
    }
}

pub fn bottom(pos: i32) -> Alignment<Vertical> {
    Alignment {
        pos,
        align: Vertical::Bottom,
    }
}

pub fn left(pos: i32) -> Alignment<Horizontal> {
    Alignment {
        pos,
        align: Horizontal::Left,
    }
}

pub fn center(pos: i32) -> Alignment<Horizontal> {
    Alignment {
        pos,
        align: Horizontal::Center,
    }
}

pub fn right(pos: i32) -> Alignment<Horizontal> {
    Alignment {
        pos,
        align: Horizontal::Right,
    }
}
