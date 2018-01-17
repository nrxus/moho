use renderer::align;

use glm;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub vertical: align::Alignment<align::Vertical>,
    pub horizontal: align::Alignment<align::Horizontal>,
}

impl Position {
    pub fn dims(self, dims: glm::UVec2) -> Destination {
        Destination {
            pos: self,
            dims: dims,
        }
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
    pub fn tl(&self) -> glm::IVec2 {
        let dims = glm::to_ivec2(self.dims);
        let top = {
            let align::Alignment { align, pos } = self.pos.vertical;
            match align {
                align::Vertical::Top => pos,
                align::Vertical::Middle => pos - dims.y / 2,
                align::Vertical::Bottom => pos - dims.y,
            }
        };
        let left = {
            let align::Alignment { align, pos } = self.pos.horizontal;
            match align {
                align::Horizontal::Left => pos,
                align::Horizontal::Center => pos - dims.x / 2,
                align::Horizontal::Right => pos - dims.x,
            }
        };
        glm::ivec2(left, top)
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
