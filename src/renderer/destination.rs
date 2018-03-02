use renderer::align;

use glm;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub vertical: align::Alignment<align::Vertical>,
    pub horizontal: align::Alignment<align::Horizontal>,
}

impl Position {
    pub fn dims(self, dims: glm::UVec2) -> Destination {
        Destination { pos: self, dims }
    }

    pub fn nudge(mut self, delta: glm::IVec2) -> Position {
        self.vertical.pos += delta.y;
        self.horizontal.pos += delta.x;
        self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Destination {
    pub pos: Position,
    pub dims: glm::UVec2,
}

impl Destination {
    pub fn top(&self) -> i32 {
        let height = self.dims.y as i32;
        let align::Alignment { align, pos } = self.pos.vertical;
        match align {
            align::Vertical::Top => pos,
            align::Vertical::Middle => pos - height / 2,
            align::Vertical::Bottom => pos - height,
        }
    }

    pub fn middle(&self) -> i32 {
        let height = self.dims.y as i32;
        let align::Alignment { align, pos } = self.pos.vertical;
        match align {
            align::Vertical::Top => pos + height / 2,
            align::Vertical::Middle => pos,
            align::Vertical::Bottom => pos - height / 2,
        }
    }

    pub fn bottom(&self) -> i32 {
        let height = self.dims.y as i32;
        let align::Alignment { align, pos } = self.pos.vertical;
        match align {
            align::Vertical::Top => pos + height,
            align::Vertical::Middle => pos + height / 2,
            align::Vertical::Bottom => pos,
        }
    }

    pub fn left(&self) -> i32 {
        let width = self.dims.x as i32;
        let align::Alignment { align, pos } = self.pos.horizontal;
        match align {
            align::Horizontal::Left => pos,
            align::Horizontal::Center => pos - width / 2,
            align::Horizontal::Right => pos - width,
        }
    }

    pub fn center(&self) -> i32 {
        let width = self.dims.x as i32;
        let align::Alignment { align, pos } = self.pos.horizontal;
        match align {
            align::Horizontal::Left => pos + width / 2,
            align::Horizontal::Center => pos,
            align::Horizontal::Right => pos - width / 2,
        }
    }

    pub fn right(&self) -> i32 {
        let width = self.dims.x as i32;
        let align::Alignment { align, pos } = self.pos.horizontal;
        match align {
            align::Horizontal::Left => pos + width,
            align::Horizontal::Center => pos + width / 2,
            align::Horizontal::Right => pos,
        }
    }

    pub fn nudge(self, nudge: glm::IVec2) -> Self {
        let pos = self.pos.nudge(nudge);
        Destination { pos, ..self }
    }

    pub fn scale(self, scale: u32) -> Destination {
        let dims = self.dims * scale;
        Destination { dims, ..self }
    }
}

impl From<glm::IVec4> for Destination {
    fn from(rect: glm::IVec4) -> Destination {
        let dims = glm::uvec2(rect.z as u32, rect.w as u32);
        align::left(rect.x).top(rect.y).dims(dims)
    }
}

impl From<glm::IVec2> for Position {
    fn from(tl: glm::IVec2) -> Position {
        align::left(tl.x).top(tl.y)
    }
}
