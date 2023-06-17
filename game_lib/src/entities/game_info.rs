use super::card::Card;

/// Available game info for player
pub struct GameInfo<'a> {
    pub campaigns: Vec<GameInfoCampaign<'a>>,
    pub main_deck_len: usize,
    pub players_hand: &'a Vec<Card>,
    pub is_game_ended: bool,
}

impl<'a> GameInfo<'a> {
    pub(crate) fn new(
        campaigns: Vec<GameInfoCampaign<'a>>,
        main_deck_len: usize,
        players_hand: &'a Vec<Card>,
        is_game_ended: bool,
    ) -> Self {
        Self {
            campaigns,
            main_deck_len,
            players_hand,
            is_game_ended,
        }
    }
}

/// Available campaign info for player
pub struct GameInfoCampaign<'a> {
    pub players_route: &'a Vec<Card>,
    pub foes_route: &'a Vec<Card>,
    pub last_free_card: Option<&'a Card>,
}

impl<'a> GameInfoCampaign<'a> {
    pub(crate) fn new(
        players_route: &'a Vec<Card>,
        foes_route: &'a Vec<Card>,
        last_free_card: Option<&'a Card>,
    ) -> Self {
        Self {
            players_route,
            foes_route,
            last_free_card,
        }
    }
}

pub struct GameInfoResults {
    pub player_1_score: isize,
    pub player_2_score: isize,
}

impl GameInfoResults {
    pub(crate) fn new(player_1_score: isize, player_2_score: isize) -> Self {
        Self {
            player_1_score,
            player_2_score,
        }
    }
}
