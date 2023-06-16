pub type CampaignType = u8;
pub type CardRank = u8;

#[derive(Debug, PartialEq, Eq)]
pub enum CardType {
    Rank(CardRank),
    HandShake,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
    Player_1,
    Player_2,
}

pub enum MoveType {
    ContinueRoute,
    MakeCardFree,
}

pub enum TakeCardFrom {
    MainDeck,
    FreeCards(CampaignType),
}
