use crate::common::types::{CampaignType, CardType};

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    campaign: CampaignType,
    card_type: CardType,
}

impl Card {
    pub(crate) fn new(campaign: CampaignType, card_type: CardType) -> Self {
        Self {
            campaign,
            card_type,
        }
    }

    pub fn campaign(&self) -> &CampaignType {
        &self.campaign
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }
}
