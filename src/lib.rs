#[macro_use]
extern crate error_chain;
extern crate glm;
extern crate num_traits;
extern crate sdl2;
extern crate take_mut;

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

pub use state::{Never, RunState, State};

pub mod errors {
    error_chain!{}
}
