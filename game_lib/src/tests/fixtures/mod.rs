use crate::{
    common::shuffler::{RandomShuffler, Shuffler},
    config::Config,
    game::Game,
};

struct NotShuffler;

impl Shuffler for NotShuffler {
    fn shuffle<T>(&self, _: &mut Vec<T>) {}
}

pub fn create_game() -> Game {
    let config = Config::new(5, 8, 3, 8, 20, 8, 20);
    let shuffler = RandomShuffler;
    Game::new(config, shuffler)
}

pub fn create_game_without_shuffle() -> Game {
    let config = Config::new(5, 8, 3, 8, 20, 8, 20);
    let shuffler = NotShuffler;
    Game::new(config, shuffler)
}
