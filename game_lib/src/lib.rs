#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

pub mod common;
mod config;
pub mod entities;

pub use config::Config;

#[cfg(not(feature = "wasm"))]
pub mod game;
#[cfg(not(feature = "wasm"))]
pub use game::Game;

#[cfg(feature = "wasm")]
pub(crate) mod game;
#[cfg(feature = "wasm")]
pub mod wasm;
#[cfg(feature = "wasm")]
pub use wasm::game::Game;

#[cfg(test)]
mod tests;

/// Usage
///
/// ```
/// use game_lib::Config;
/// use game_lib::GameBuilder;
///
/// ...
///
/// let config = Config::new(...);
/// let game = GameBuilder::build(config);
/// ```
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GameBuilder;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl GameBuilder {
    #[cfg_attr(feature = "wasm", wasm_bindgen)]
    pub fn build(config: config::Config) -> Game {
        let shuffler = common::shuffler::RandomShuffler;
        Game::new(config, shuffler)
    }
}
