use wasm_bindgen::prelude::*;

use lost_cities_game_lib::{
    common::types::{MoveType, Player as InnerPlayer, TakeCardFrom},
    config::Config as InnerConfig,
    entities::game_info::{GameInfo as InnerGameInfo, GameInfoResults as InnerGameInfoResults},
    game::Game as InnerGame,
    GameBuilder,
};

type GameResult<T> = Result<T, JsError>;

#[wasm_bindgen]
pub struct Game {
    inner: InnerGame,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(config: Config) -> Self {
        let inner = GameBuilder::build(&config.into());
        Self { inner }
    }

    pub fn make_move(
        &mut self,
        player: Player,
        card_index: usize,
        is_continue_route: bool,
        is_take_from_main_deck: bool,
        free_cards_campaign: Option<u8>,
    ) -> GameResult<()> {
        let move_type = if is_continue_route {
            MoveType::ContinueRoute
        } else {
            MoveType::ContinueRoute
        };

        let take_card_from = if is_take_from_main_deck {
            TakeCardFrom::MainDeck
        } else {
            TakeCardFrom::FreeCards(free_cards_campaign.unwrap())
        };

        self.inner
            .make_move(&player.into(), card_index, move_type, take_card_from)?;
        Ok(())
    }

    pub fn game_info(&self, player: Player) -> GameInfo {
        todo!()
    }

    #[wasm_bindgen(method, getter)]
    pub fn game_results(&self) -> GameInfoResults {
        todo!()
    }
}

#[wasm_bindgen]
pub struct Config {
    pub campaigns_amount: u8,
    pub card_ranks_amount: u8,
    pub handshakes_amount: u8,
    pub cards_on_hand_amount: u8,
    pub campaign_outcome: usize,
}

impl From<Config> for InnerConfig {
    fn from(value: Config) -> Self {
        Self {
            campaigns_amount: value.campaigns_amount,
            card_ranks_amount: value.card_ranks_amount,
            handshakes_amount: value.handshakes_amount,
            cards_on_hand_amount: value.cards_on_hand_amount,
            campaign_outcome: value.campaign_outcome,
        }
    }
}

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

#[wasm_bindgen]
pub struct GameInfo {}

#[wasm_bindgen]
pub struct GameInfoResults {}
