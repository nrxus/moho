#[derive(Debug, Clone, Copy)]
pub enum Vertical {
    Top,
    Middle,
    Bottom,
}

#[derive(Debug, Clone, Copy)]
pub enum Horizontal {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy)]
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
    pub fn left(self, pos: i32) -> super::Destination {
        let horizontal = Alignment {
            pos,
            align: Horizontal::Left,
        };
        super::Destination {
            horizontal,
            vertical: self,
            dims: None,
        }
    }

    pub fn center(self, pos: i32) -> super::Destination {
        let horizontal = Alignment {
            pos,
            align: Horizontal::Center,
        };
        super::Destination {
            horizontal,
            vertical: self,
            dims: None,
        }
    }

    pub fn right(self, pos: i32) -> super::Destination {
        let horizontal = Alignment {
            pos,
            align: Horizontal::Right,
        };
        super::Destination {
            horizontal,
            vertical: self,
            dims: None,
        }
    }
}

impl Alignment<Horizontal> {
    pub fn top(self, pos: i32) -> super::Destination {
        let vertical = Alignment {
            pos,
            align: Vertical::Top,
        };
        super::Destination {
            vertical,
            horizontal: self,
            dims: None,
        }
    }

    pub fn middle(self, pos: i32) -> super::Destination {
        let vertical = Alignment {
            pos,
            align: Vertical::Middle,
        };
        super::Destination {
            vertical,
            horizontal: self,
            dims: None,
        }
    }

    pub fn bottom(self, pos: i32) -> super::Destination {
        let vertical = Alignment {
            pos,
            align: Vertical::Bottom,
        };
        super::Destination {
            vertical,
            horizontal: self,
            dims: None,
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
