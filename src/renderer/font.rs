use errors::*;
use super::{ColorRGBA, Loader};

use glm;

pub trait Font {
    type Texture;

    fn measure(&self, text: &str) -> Result<glm::UVec2>;
    fn texturize(&self, text: &str, color: &ColorRGBA) -> Result<Self::Texture>;
}

pub trait FontLoader<'a>
    : Loader<'a, <Self as FontLoader<'a>>::Font, Args = Details> {
    type Font: Font;
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Details {
    pub path: &'static str,
    pub size: u16,
}

impl<'a> From<&'a Details> for Details {
    fn from(details: &'a Details) -> Details {
        details.clone()
    }
}
