use crate::common::types::{CampaignType, CardType};

#[cfg_attr(feature = "wasm", derive(Clone))]
#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    id: usize,
    campaign: CampaignType,
    card_type: CardType,
}

impl Card {
    pub(crate) fn new(id: usize, campaign: CampaignType, card_type: CardType) -> Self {
        Self {
            id,
            campaign,
            card_type,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn campaign(&self) -> &CampaignType {
        &self.campaign
    }

    pub fn card_type(&self) -> &CardType {
        &self.card_type
    }
}
