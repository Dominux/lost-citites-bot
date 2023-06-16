use std::hash::Hash;

use crate::common::types::CampaignType;

use super::card::Card;

pub struct Campaign {
    campaign_type: CampaignType,
    player_1_route: Vec<Card>,
    player_2_route: Vec<Card>,
    free_cards: Vec<Card>,
}

impl Hash for Campaign {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.campaign_type.hash(state);
    }
}

impl PartialEq for Campaign {
    fn eq(&self, other: &Self) -> bool {
        self.campaign_type == other.campaign_type
    }
}

impl Eq for Campaign {}
