use wasm_bindgen::prelude::*;

use crate::{
    common::{
        shuffler::RandomShuffler,
        types::{MoveType, TakeCardFrom},
    },
    config::Config,
    entities::game_info::GameInfoResults,
    game::Game as InnerGame,
};

use super::types::{GameInfo, GameResult, Player};

#[wasm_bindgen]
pub struct Game {
    inner: InnerGame,
}

#[wasm_bindgen]
impl Game {
    pub(crate) fn new(config: Config, shuffler: RandomShuffler) -> Self {
        let inner = InnerGame::new(config, shuffler);
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
            MoveType::ContinueCampaignRoute
        } else {
            MoveType::ContinueCampaignRoute
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
        self.inner.game_info(&player.into()).into()
    }

    #[wasm_bindgen(getter)]
    pub fn game_results(&self) -> GameInfoResults {
        self.inner.game_results()
    }
}
