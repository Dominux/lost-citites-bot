pub mod common;
pub mod config;
pub mod entities;
pub mod game;

#[cfg(test)]
mod tests;

/// Usage
///
/// ```
/// use game_lib::config::Config;
/// use game_lib::GameBuilder;
///
/// ...
///
/// let config = Config::new(...);
/// let game = GameBuilder::build(config);
/// ```
pub struct GameBuilder;

impl GameBuilder {
    pub fn build(config: &config::Config) -> game::Game {
        let shuffler = common::shuffler::RandomShuffler;
        game::Game::new(config, shuffler)
    }
}
