use super::{campaign::Campaign, card::Card};

pub struct Playground {
    pub(crate) campaigns: Vec<Campaign>,
    pub(crate) main_deck: Vec<Card>,
}

impl Playground {
    pub(crate) fn new(campaigns: Vec<Campaign>, main_deck: Vec<Card>) -> Self {
        Self {
            campaigns,
            main_deck,
        }
    }
}
