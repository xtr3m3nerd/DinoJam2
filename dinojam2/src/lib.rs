#[cfg(target_arch = "wasm32")]
mod web_main;

mod game;

pub mod asset_management;
pub mod config;
pub mod debug;
pub mod map;
pub mod plugins;
pub mod scenes;
pub mod states;
pub mod util;

pub use game::*;
