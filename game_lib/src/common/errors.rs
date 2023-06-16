use super::types::{CampaignType, Player};

#[derive(thiserror::Error, Debug)]
pub enum GameError {
    #[error("Now it is a [0] turn")]
    NotPlayersPlayer(Player),
    #[error("Player does not have such a card")]
    PlayerDoesNotHaveSuchCard,
    #[error("Last card in the route has greater rank")]
    LastCardHasGreaterRank,
    #[error("You cannot put and take the same card within a move")]
    PuttingAndTakingSameCard,
    #[error("Campaign {0} does not exist")]
    CampaignDoesNotExist(CampaignType),
    #[error("Campaign {0} does not have free cards")]
    CampaignDoesNotHaveFreeCards(CampaignType),
    #[error("Game is over")]
    GameIsOver,
}

pub type GameResult<T> = Result<T, GameError>;
