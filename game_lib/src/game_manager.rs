use crate::{
    common::{
        errors::GameResult,
        types::{MoveType, Player, TakeCardFrom},
    },
    entities::game_info::GameInfo,
    game::Game,
};

pub struct GameMoveParams<'a> {
    player: &'a Player,
    card_id: usize,
    move_type: MoveType,
    take_card_from: TakeCardFrom,
}

pub trait GamePlayer {
    fn make_move(&self, game_info: &GameInfo) -> GameResult<GameMoveParams>;
}

pub struct GameManager<T1: GamePlayer, T2: GamePlayer> {
    game: Game,
    game_player1: T1,
    game_player2: T2,
}

impl<T1: GamePlayer, T2: GamePlayer> GameManager<T1, T2> {
    pub fn new(game: Game, game_player1: T1, game_player2: T2) -> Self {
        Self {
            game,
            game_player1,
            game_player2,
        }
    }

    pub fn run_game_process(&mut self) {
        loop {
            // Player 1
            {
                let info = self.game.game_info(&Player::Player1);
                let params = self.game_player1.make_move(&info).unwrap();
                self.game
                    .make_move(
                        params.player,
                        params.card_id,
                        params.move_type,
                        params.take_card_from,
                    )
                    .unwrap();
            }

            // Player 2
            {
                let info = self.game.game_info(&Player::Player2);
                let params = self.game_player2.make_move(&info).unwrap();
                self.game
                    .make_move(
                        params.player,
                        params.card_id,
                        params.move_type,
                        params.take_card_from,
                    )
                    .unwrap();
            }
        }
    }
}
