use crate::{
    common::{
        errors::{GameError, GameResult},
        types::{MoveType, Player, TakeCardFrom},
    },
    tests::fixtures::create_game_without_shuffle,
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

#[test]
fn test_make_move_by_wrong_player() {
    let mut game = create_game();

    let player = Player::Player2;
    let info = game.game_info(&player);
    assert!(!info.is_players_turn);

    let move_result = game.make_move(&player, 0, MoveType::MakeCardFree, TakeCardFrom::MainDeck);
    assert!(
        matches!(&move_result, GameResult::Err(err) if matches!(&err, GameError::NotPlayersTurn(turn) if Player::Player1 == *turn))
    )
}

#[test]
fn test_take_card_from_empty_free_cards() {
    let mut game = create_game_without_shuffle();

    let player = Player::Player1;
    let campaign_to_take_free_card = 0_u8;
    let info = game.game_info(&player);
    assert!(info.campaigns[campaign_to_take_free_card as usize]
        .last_free_card
        .is_none());

    let move_result = game.make_move(
        &player,
        (*info.players_hand[0].campaign()).into(),
        MoveType::MakeCardFree,
        TakeCardFrom::FreeCards(campaign_to_take_free_card),
    );
    assert!(
        matches!(&move_result, GameResult::Err(err) if matches!(&err, GameError::CampaignDoesNotHaveFreeCards(campaign_type) if *campaign_type == campaign_to_take_free_card))
    )
}
