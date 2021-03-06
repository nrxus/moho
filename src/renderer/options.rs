use super::Destination;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rotation {
    pub angle: f64,
    pub center: glm::IVec2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Flip {
    Horizontal,
    Vertical,
    Both,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Options {
    pub dst: Option<Destination>,
    pub src: Option<glm::UVec4>,
    pub rotation: Option<Rotation>,
    pub flip: Option<Flip>,
}

impl Options {
    pub fn at(mut self, dst: impl Into<Destination>) -> Self {
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

pub fn at(dst: impl Into<Destination>) -> Options {
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
