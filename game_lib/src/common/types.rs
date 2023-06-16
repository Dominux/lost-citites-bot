pub type CampaignType = u8;
pub type CardRank = u8;

pub enum CardType {
    Rank(CardRank),
    HandShake,
}
