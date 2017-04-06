use errors::*;
use super::{ColorRGBA, Loader, Texture};

use glm;

pub trait Font {
    fn measure(&self, text: &str) -> Result<glm::UVec2>;
}

pub trait FontLoader<'a>
    : Loader<'a, <Self as FontLoader<'a>>::Font, FontDetails> {
    type Font: Font;
}

pub trait FontTexturizer<'a> {
    type Texture: Texture;
    type Font: Font;
    fn texturize(&self, font: &Self::Font, text: &str, color: ColorRGBA) -> Result<Self::Texture>;
}

#[derive(PartialEq, Eq, Hash)]
pub struct FontDetails {
    pub path: &'static str,
    pub size: u16,
}

impl<'a> From<&'a FontDetails> for FontDetails {
    fn from(details: &'a FontDetails) -> FontDetails {
        FontDetails {
            path: details.path,
            size: details.size,
        }
    }
}
