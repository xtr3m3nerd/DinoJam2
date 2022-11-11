#[cfg(target_arch = "wasm32")]
mod web_main;

mod game;

pub mod config;
pub mod debug;
pub mod scenes;

pub use game::*;
