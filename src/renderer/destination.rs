use renderer::align;

use glm;

#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub vertical: align::Alignment<align::Vertical>,
    pub horizontal: align::Alignment<align::Horizontal>,
}

impl Position {
    pub fn dims(self, dims: glm::UVec2) -> Destination {
        let top = {
            let align::Alignment { align, pos } = self.vertical;
            match align {
                align::Vertical::Top => pos,
                align::Vertical::Middle => pos - (dims.y / 2) as i32,
                align::Vertical::Bottom => pos - dims.y as i32,
            }
        };
        let left = {
            let align::Alignment { align, pos } = self.horizontal;
            match align {
                align::Horizontal::Left => pos,
                align::Horizontal::Center => pos - (dims.x / 2) as i32,
                align::Horizontal::Right => pos - dims.x as i32,
            }
        };
        Destination {
            tl: glm::ivec2(top, left),
            dims,
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
    pub tl: glm::IVec2,
    pub dims: glm::UVec2,
}

impl Destination {
    pub fn nudge(self, nudge: glm::IVec2) -> Self {
        let tl = self.tl + nudge;
        Destination { tl, ..self }
    }

    pub fn scale(self, scale: u32) -> Destination {
        let dims = self.dims * scale;
        Destination { dims, ..self }
    }
}

impl From<glm::IVec4> for Destination {
    fn from(rect: glm::IVec4) -> Destination {
        Destination {
            tl: glm::ivec2(rect.x, rect.y),
            dims: glm::uvec2(rect.z as u32, rect.w as u32),
        }
    }
}

impl From<glm::IVec2> for Position {
    fn from(tl: glm::IVec2) -> Position {
        align::left(tl.x).top(tl.y)
    }
}
