use crate::{
    common::shuffler::{NotShuffler, RandomShuffler},
    config::Config,
    game::Game,
};

pub fn create_game() -> Game {
    let config = Config::default();
    let shuffler = RandomShuffler;
    Game::new(config, shuffler)
}

pub fn create_game_without_shuffle() -> Game {
    let config = Config::default();
    let shuffler = NotShuffler;
    Game::new(config, shuffler)
}
