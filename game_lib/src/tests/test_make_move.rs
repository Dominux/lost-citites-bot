use crate::common::{
    errors::{GameError, GameResult},
    types::{MoveType, Player, TakeCardFrom},
};

use super::fixtures::create_game;

#[test]
fn test_make_move_after_game_over() {
    let mut game = create_game();
    // Ending game
    game.playground.main_deck.clear();

    let player = Player::Player1;
    let info = game.game_info(&player);
    assert!(info.is_game_ended);

    let move_result = game.make_move(&player, 0, MoveType::MakeCardFree, TakeCardFrom::MainDeck);
    assert!(matches!(move_result, GameResult::Err(err) if matches!(err, GameError::GameIsOver)))
}
