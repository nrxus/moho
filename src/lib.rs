pub mod animation;
pub mod engine;
pub mod font;
pub mod input;
pub mod renderer;
pub mod resource;
pub mod sdl2_helpers;
pub mod shape;
pub mod texture;
pub mod timer;
pub mod window_wrapper;

mod state;

pub use crate::state::{Never, RunState, State};
pub use moho_derive::*;

pub type Result<T> = std::result::Result<T, failure::Error>;
