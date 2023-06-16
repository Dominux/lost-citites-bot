use crate::common::types::CampaignType;

use super::card::Card;

pub struct Campaign {
    pub(crate) campaign_type: CampaignType,
    pub(crate) player_1_route: Vec<Card>,
    pub(crate) player_2_route: Vec<Card>,
    pub(crate) free_cards: Vec<Card>,
}

impl Campaign {
    pub(crate) fn new(
        campaign_type: CampaignType,
        player_1_route: Vec<Card>,
        player_2_route: Vec<Card>,
        free_cards: Vec<Card>,
    ) -> Self {
        Self {
            campaign_type,
            player_1_route,
            player_2_route,
            free_cards,
        }
    }
}
