use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    common::types::{CampaignType, CardType, Player as InnerPlayer},
    entities::card::Card as InnerCard,
};

#[wasm_bindgen]
#[derive(Debug, Clone, Copy)]
pub enum Player {
    Player1 = "Player1",
    Player2 = "Player2",
}

impl From<Player> for InnerPlayer {
    fn from(value: Player) -> Self {
        match value {
            Player::Player1 => Self::Player1,
            Player::Player2 => Self::Player2,
            _ => panic!("Wrong invariant"),
        }
    }
}

pub type GameResult<T> = Result<T, JsError>;

/// Available game info for player
#[wasm_bindgen]
pub struct GameInfo {
    campaigns: Vec<GameInfoCampaign>,
    pub is_players_turn: bool,
    pub main_deck_len: usize,
    players_hand: Vec<Card>,
    pub is_game_ended: bool,
}

#[wasm_bindgen]
impl GameInfo {
    pub(crate) fn new(
        campaigns: Vec<GameInfoCampaign>,
        is_players_turn: bool,
        main_deck_len: usize,
        players_hand: Vec<Card>,
        is_game_ended: bool,
    ) -> Self {
        Self {
            campaigns,
            is_players_turn,
            main_deck_len,
            players_hand,
            is_game_ended,
        }
    }

    #[wasm_bindgen(getter)]
    pub fn campaigns(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.campaigns).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn players_hand(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.players_hand).unwrap()
    }
}

/// Available campaign info for player
#[wasm_bindgen]
#[derive(Serialize)]
pub struct GameInfoCampaign {
    players_route: Vec<Card>,
    foes_route: Vec<Card>,
    pub last_free_card: Option<Card>,
}

impl GameInfoCampaign {
    pub(crate) fn new(
        players_route: Vec<Card>,
        foes_route: Vec<Card>,
        last_free_card: Option<Card>,
    ) -> Self {
        Self {
            players_route,
            foes_route,
            last_free_card,
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Card {
    campaign: CampaignType,
    card_type: CardType,
}

#[wasm_bindgen]
impl Card {
    #[wasm_bindgen(getter)]
    pub fn campaign(&self) -> CampaignType {
        self.campaign
    }

    #[wasm_bindgen(getter)]
    pub fn card_type(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.card_type).unwrap()
    }
}

impl From<InnerCard> for Card {
    fn from(value: InnerCard) -> Self {
        Self {
            campaign: *value.campaign(),
            card_type: *value.card_type(),
        }
    }
}
