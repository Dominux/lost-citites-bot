pub struct Config {
    pub campaigns_amount: u8,
    pub card_ranks_amount: u8,
    pub handshakes_amount: u8,
    pub cards_on_hand_amount: u8,
    pub campaign_outcome: usize,
}

impl Config {
    pub fn new(
        campaigns_amount: u8,
        card_ranks_amount: u8,
        handshakes_amount: u8,
        cards_on_hand_amount: u8,
        campaign_outcome: usize,
    ) -> Self {
        Self {
            campaigns_amount,
            card_ranks_amount,
            handshakes_amount,
            cards_on_hand_amount,
            campaign_outcome,
        }
    }
}
