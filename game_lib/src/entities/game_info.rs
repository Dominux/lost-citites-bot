#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

use super::card::Card;

/// Available game info for player
pub struct GameInfo<'a> {
    pub campaigns: Vec<GameInfoCampaign<'a>>,
    pub is_players_turn: bool,
    pub main_deck_len: usize,
    pub players_hand: &'a Vec<Card>,
    pub is_game_ended: bool,
}

/// Available campaign info for player
pub struct GameInfoCampaign<'a> {
    pub players_route: &'a Vec<Card>,
    pub foes_route: &'a Vec<Card>,
    pub last_free_card: Option<&'a Card>,
}

impl<'a> GameInfoCampaign<'a> {
    pub(crate) fn new(
        players_route: &'a Vec<Card>,
        foes_route: &'a Vec<Card>,
        last_free_card: Option<&'a Card>,
    ) -> Self {
        Self {
            players_route,
            foes_route,
            last_free_card,
        }
    }
}

#[cfg_attr(feature = "wasm", wasm_bindgen, derive(Debug, Clone, Copy))]
pub struct GameInfoResults {
    pub player_1_score: isize,
    pub player_2_score: isize,
}
