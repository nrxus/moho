extern crate failure;
extern crate glm;
extern crate num_traits;
extern crate sdl2;

#[macro_use]
extern crate moho_derive;
pub use moho_derive::*;

pub mod animation;
pub mod engine;
pub mod font;
pub mod input;
pub mod renderer;
pub mod resource;
pub mod sdl2_helpers;
pub mod shape;
mod state;
pub mod texture;
pub mod timer;
pub mod window_wrapper;

use failure::Error;

pub use state::{Never, RunState, State};

pub type Result<T> = std::result::Result<T, Error>;
