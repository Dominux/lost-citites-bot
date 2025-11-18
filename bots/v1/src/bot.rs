use lost_cities_game_lib::{
    Config, GameMoveParams, GamePlayer, common::errors::GameResult, entities::game_info::GameInfo,
};

pub struct Bot<'c> {
    config: &'c Config,
}

impl<'c> Bot<'c> {
    pub fn new(config: &'c Config) -> Self {
        Self { config }
    }
}

impl GamePlayer for Bot<'_> {
    fn make_move(&self, game_info: &GameInfo) -> GameResult<GameMoveParams> {
        todo!()
    }
}
