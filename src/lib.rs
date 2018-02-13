extern crate failure;
extern crate glm;
extern crate num_traits;
extern crate sdl2;

mod state;
pub mod sdl2_helpers;
pub mod animation;
pub mod input;
pub mod shape;
pub mod renderer;
pub mod timer;
pub mod window_wrapper;
pub mod engine;
pub mod resource;
pub mod texture;
pub mod font;

use failure::Error;

pub use state::{Never, RunState, State};

pub type Result<T> = std::result::Result<T, Error>;
