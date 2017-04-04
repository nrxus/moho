use errors::*;
use super::{ColorRGBA, Loader, Resource, Texture};

pub trait Font: Resource {}

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

pub trait FontLoader<'a, T: Font>: Loader<'a, T, LoadData = FontDetails> {}

pub trait FontTexturizer<'a, F: Font> {
    type Texture: Texture;
    fn texturize(&self, font: &F, text: &str, color: ColorRGBA) -> Result<Self::Texture>;
}
