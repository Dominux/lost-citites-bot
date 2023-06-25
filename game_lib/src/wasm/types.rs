use serde::Serialize;
use wasm_bindgen::prelude::*;

use crate::{
    common::types::{CampaignType, CardType, Player as InnerPlayer},
    entities::{
        card::Card as InnerCard,
        game_info::{GameInfo as InnerGameInfo, GameInfoCampaign as InnerGameInfoCampaign},
    },
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
    #[wasm_bindgen(getter)]
    pub fn campaigns(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.campaigns).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn players_hand(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.players_hand).unwrap()
    }
}

impl<'a> From<InnerGameInfo<'a>> for GameInfo {
    fn from(inner: InnerGameInfo) -> Self {
        Self {
            campaigns: inner
                .campaigns
                .into_iter()
                .map(|campaign| campaign.into())
                .collect(),
            is_players_turn: inner.is_players_turn,
            main_deck_len: inner.main_deck_len,
            players_hand: inner
                .players_hand
                .into_iter()
                .map(|card| card.clone().into())
                .collect(),
            is_game_ended: inner.is_game_ended,
        }
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

#[wasm_bindgen]
impl GameInfoCampaign {
    #[wasm_bindgen(getter)]
    pub fn players_route(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.players_route).unwrap()
    }

    #[wasm_bindgen(getter)]
    pub fn foes_route(&self) -> JsValue {
        serde_wasm_bindgen::to_value(&self.foes_route).unwrap()
    }
}

impl<'a> From<InnerGameInfoCampaign<'a>> for GameInfoCampaign {
    fn from(inner: InnerGameInfoCampaign) -> Self {
        Self {
            players_route: inner
                .players_route
                .into_iter()
                .map(|card| card.clone().into())
                .collect(),
            foes_route: inner
                .foes_route
                .into_iter()
                .map(|card| card.clone().into())
                .collect(),
            last_free_card: inner.last_free_card.map(|card| card.clone().into()),
        }
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone, Copy, Serialize)]
pub struct Card {
    campaign: CampaignType,
    card_type: CardType,
    id: usize,
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

    #[wasm_bindgen(getter)]
    pub fn id(&self) -> usize {
        self.id
    }
}

impl From<InnerCard> for Card {
    fn from(value: InnerCard) -> Self {
        Self {
            campaign: *value.campaign(),
            card_type: *value.card_type(),
            id: value.id(),
        }
    }
}
