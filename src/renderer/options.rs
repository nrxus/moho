use super::align;
use glm;

#[derive(Debug, Clone, Copy)]
pub struct Rotation {
    pub angle: f64,
    pub center: glm::IVec2,
}

#[derive(Debug, Clone, Copy)]
pub enum Flip {
    Horizontal,
    Vertical,
    Both,
}

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

#[derive(Debug, Default, Clone)]
pub struct Options {
    pub dst: Option<Destination>,
    pub src: Option<glm::UVec4>,
    pub rotation: Option<Rotation>,
    pub flip: Option<Flip>,
}

impl Options {
    pub fn at<D: Into<Destination>>(mut self, dst: D) -> Self {
        self.dst = Some(dst.into());
        self
    }

    pub fn from(mut self, src: glm::UVec4) -> Self {
        self.src = Some(src);
        self
    }

    pub fn flip(mut self, flip: Flip) -> Self {
        self.flip = Some(flip);
        self
    }

    pub fn rotate(mut self, rotation: Rotation) -> Self {
        self.rotation = Some(rotation);
        self
    }
}

pub fn none() -> Options {
    Options::default()
}

pub fn at<D: Into<Destination>>(dst: D) -> Options {
    Options::default().at(dst)
}

pub fn from(src: glm::UVec4) -> Options {
    Options::default().from(src)
}

pub fn flip(flip: Flip) -> Options {
    Options::default().flip(flip)
}

pub fn rotate(rotation: Rotation) -> Options {
    Options::default().rotate(rotation)
}
