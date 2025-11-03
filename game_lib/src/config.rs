#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen, derive(Debug, Clone, Copy))]
pub struct Config {
    pub campaigns_amount: u8,
    pub card_ranks_amount: u8,
    pub handshakes_amount: u8,
    pub cards_on_hand_amount: u8,
    pub campaign_outcome: usize,
    pub min_cards_for_bonus: u8,
    pub bonus: u8,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
impl Config {
    #[cfg_attr(feature = "wasm", wasm_bindgen(constructor))]
    pub fn new(
        campaigns_amount: u8,
        card_ranks_amount: u8,
        handshakes_amount: u8,
        cards_on_hand_amount: u8,
        campaign_outcome: usize,
        min_cards_for_bonus: u8,
        bonus: u8,
    ) -> Self {
        Self {
            campaigns_amount,
            card_ranks_amount,
            handshakes_amount,
            cards_on_hand_amount,
            campaign_outcome,
            min_cards_for_bonus,
            bonus,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            campaigns_amount: 5,
            card_ranks_amount: 8,
            handshakes_amount: 3,
            cards_on_hand_amount: 8,
            campaign_outcome: 20,
            min_cards_for_bonus: 8,
            bonus: 20,
        }
    }
}
