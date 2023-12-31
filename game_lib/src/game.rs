use crate::{
    common::{
        errors::{GameError, GameResult},
        shuffler::Shuffler,
        types::{CampaignType, CardType, MoveType, Player, TakeCardFrom},
    },
    config::Config,
    entities::{
        campaign::Campaign,
        card::Card,
        game_info::{GameInfo, GameInfoCampaign, GameInfoResults},
        playground::Playground,
    },
};

pub struct Game {
    pub(crate) playground: Playground,
    turn: Player,
    player_1_hand: Vec<Card>,
    player_2_hand: Vec<Card>,
    config: Config,
}

impl Game {
    pub(crate) fn new<T: Shuffler>(config: Config, shuffler: T) -> Self {
        // generating cards
        let mut next_id = 0;
        let mut cards: Vec<_> = (0..config.campaigns_amount)
            .flat_map(|campaign_type| {
                let mut cards = Vec::with_capacity(
                    (config.card_ranks_amount + config.handshakes_amount) as usize,
                );

                for rank in 1..=config.card_ranks_amount {
                    let card = Card::new(next_id, campaign_type, CardType::Rank(rank));
                    cards.push(card);
                    next_id += 1;
                }
                for _ in 0..config.handshakes_amount {
                    let card = Card::new(next_id, campaign_type, CardType::HandShake);
                    cards.push(card);
                    next_id += 1;
                }
                cards
            })
            .collect();

        // shuffling cards
        shuffler.shuffle(&mut cards);

        // taking initial cards for players
        let player_1_hand = cards.split_off(cards.len() - config.cards_on_hand_amount as usize);
        let player_2_hand = cards.split_off(cards.len() - config.cards_on_hand_amount as usize);

        // generating campaigns
        let campaigns = {
            let campaign_cards_amount =
                config.card_ranks_amount as usize + config.handshakes_amount as usize;
            (0..config.campaigns_amount)
                .map(|campaign_type| {
                    Campaign::new(
                        campaign_type,
                        Vec::with_capacity(campaign_cards_amount),
                        Vec::with_capacity(campaign_cards_amount),
                        Vec::with_capacity(campaign_cards_amount),
                    )
                })
                .collect()
        };

        // generating playground
        let playground = Playground::new(campaigns, cards);

        Self {
            playground,
            turn: Player::Player1,
            player_1_hand,
            player_2_hand,
            config,
        }
    }

    pub fn make_move(
        &mut self,
        player: &Player,
        card_id: usize,
        move_type: MoveType,
        take_card_from: TakeCardFrom,
    ) -> GameResult<()> {
        // if game is ended
        if self.is_game_ended() {
            return Err(GameError::GameIsOver);
        }

        // turn validation
        if *player != self.turn {
            return Err(GameError::NotPlayersTurn(self.turn.clone()));
        }

        // some validation
        if let TakeCardFrom::FreeCards(campaign_type) = take_card_from {
            // if free cards has a campaign type, we may not have it
            let campaign =
                Self::get_campaign_by_type(&mut self.playground.campaigns, &campaign_type)?;

            // if campaign's free cards is empty
            if campaign.free_cards.is_empty() {
                return Err(GameError::CampaignDoesNotHaveFreeCards(campaign_type));
            }
        }

        let players_hand = match self.turn {
            Player::Player1 => &mut self.player_1_hand,
            Player::Player2 => &mut self.player_2_hand,
        };

        let (card_type, campaign_type) = {
            let card = players_hand
                .iter()
                .find(|card| card.id() == card_id)
                .ok_or(GameError::PlayerDoesNotHaveSuchCard)?;
            (card.card_type(), card.campaign())
        };

        // getting choosen campaign
        let campaign =
            Self::get_campaign_by_type(&mut self.playground.campaigns, campaign_type).unwrap(); // exactly we created campaign types inside cards

        if matches!(move_type, MoveType::ContinueCampaignRoute) {
            // continuing route
            let route = match player {
                Player::Player1 => &mut campaign.player_1_route,
                Player::Player2 => &mut campaign.player_2_route,
            };

            if let Some(last_card) = route.last() {
                if let CardType::Rank(last_rank) = last_card.card_type() {
                    if !matches!(card_type, CardType::Rank(rank) if rank > last_rank) {
                        // if last card in the route is ranked and has greater rank than the chosen card's one
                        return Err(GameError::LastCardHasGreaterRank);
                    }
                }
            }

            // appending the card to the route
            Self::move_card_from_hands_to_vec(card_id, players_hand, route)
        } else {
            // making card free

            if matches!(take_card_from, TakeCardFrom::FreeCards(campaign_type) if campaign_type == campaign.campaign_type)
            {
                // if player trying to put and take the same card
                return Err(GameError::PuttingAndTakingSameCard);
            }

            // appending the card to the free cards
            Self::move_card_from_hands_to_vec(card_id, players_hand, &mut campaign.free_cards)
        }

        let new_card = match take_card_from {
            TakeCardFrom::MainDeck => {
                self.playground.main_deck.pop().unwrap() // we won't reach that point
            }
            TakeCardFrom::FreeCards(campaign_type) => {
                let campaign =
                    Self::get_campaign_by_type(&mut self.playground.campaigns, &campaign_type)
                        .unwrap(); // we already validated it
                campaign.free_cards.pop().unwrap() // and this too
            }
        };

        players_hand.push(new_card);

        // changing turn
        self.turn = match self.turn {
            Player::Player1 => Player::Player2,
            Player::Player2 => Player::Player1,
        };

        Ok(())
    }

    #[inline]
    fn get_campaign_by_type<'a>(
        campaigns: &'a mut [Campaign],
        campaign_type: &CampaignType,
    ) -> GameResult<&'a mut Campaign> {
        campaigns
            .iter_mut()
            .find(|campaign| *campaign_type == campaign.campaign_type)
            .ok_or(GameError::CampaignDoesNotExist(*campaign_type))
    }

    #[inline]
    fn move_card_from_hands_to_vec(
        card_id: usize,
        hand: &mut Vec<Card>,
        target_vec: &mut Vec<Card>,
    ) {
        let index = hand
            .iter_mut()
            .position(|card| card.id() == card_id)
            .unwrap();
        let card = hand.swap_remove(index);
        target_vec.push(card)
    }

    #[inline]
    fn is_game_ended(&self) -> bool {
        self.playground.main_deck.is_empty()
    }

    pub fn game_info(&self, player: &Player) -> GameInfo {
        let players_hand = match player {
            Player::Player1 => &self.player_1_hand,
            Player::Player2 => &self.player_2_hand,
        };

        let main_deck_len = self.playground.main_deck.len();
        let is_game_ended = self.is_game_ended();

        let campaigns = self
            .playground
            .campaigns
            .iter()
            .map(|campaign| {
                let (players_route, foes_route) = match player {
                    Player::Player1 => (&campaign.player_1_route, &campaign.player_2_route),
                    Player::Player2 => (&campaign.player_2_route, &campaign.player_1_route),
                };

                let last_free_card = campaign.free_cards.last();
                GameInfoCampaign::new(players_route, foes_route, last_free_card)
            })
            .collect();

        let is_players_turn = self.turn == *player;

        GameInfo {
            campaigns,
            is_players_turn,
            main_deck_len,
            players_hand,
            is_game_ended,
        }
    }

    pub fn game_results(&self) -> GameInfoResults {
        let (mut player_1_score, mut player_2_score) = (0, 0);

        for campaign in self.playground.campaigns.iter() {
            for (route, players_score) in [
                (&campaign.player_1_route, &mut player_1_score),
                (&campaign.player_2_route, &mut player_2_score),
            ] {
                let (score, multiplier) =
                    route
                        .iter()
                        .fold((0, 1), |(score, multiplier), card| match card.card_type() {
                            CardType::HandShake => (score, multiplier + 1),
                            CardType::Rank(rank) => (score + *rank as isize, multiplier),
                        });

                // calculating route income
                let mut route_income = (score - self.config.campaign_outcome as isize) * multiplier;
                if route.len() >= self.config.min_cards_for_bonus as usize {
                    route_income += self.config.bonus as isize
                }
                *players_score += route_income;
            }
        }

        GameInfoResults {
            player_1_score,
            player_2_score,
        }
    }
}
