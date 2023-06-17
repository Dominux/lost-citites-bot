use crate::{common::shuffler::RandomShuffler, config::Config, game::Game};

pub fn create_game() -> Game {
    let config = Config::new(5, 8, 3, 8, 20);
    let shuffler = RandomShuffler {};
    Game::new(&config, shuffler)
}
