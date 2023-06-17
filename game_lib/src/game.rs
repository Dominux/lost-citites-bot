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
    playground: Playground,
    turn: Player,
    player_1_hand: Vec<Card>,
    player_2_hand: Vec<Card>,
    campaign_outcome: usize,
}

impl Game {
    pub fn new<T: Shuffler>(config: &Config, shuffler: T) -> Self {
        // generating cards
        let mut cards: Vec<_> = (0..config.campaigns_amount)
            .flat_map(|campaign_type| {
                let rank_cards = (1..=config.card_ranks_amount)
                    .map(move |rank| Card::new(campaign_type, CardType::Rank(rank)));
                let handshake_cards = (1..=config.handshakes_amount)
                    .map(move |_| Card::new(campaign_type, CardType::HandShake));
                rank_cards.chain(handshake_cards)
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
            campaign_outcome: config.campaign_outcome,
        }
    }

    pub fn make_move(
        &mut self,
        player: &Player,
        card: &Card,
        move_type: MoveType,
        take_card_from: TakeCardFrom,
    ) -> GameResult<()> {
        // if game is ended
        if self.is_game_ended() {
            return Err(GameError::GameIsOver);
        }

        // turn validation
        if *player == self.turn {
            return Err(GameError::NotPlayersPlayer(self.turn.clone()));
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

        // getting choosen campaign
        let campaign =
            Self::get_campaign_by_type(&mut self.playground.campaigns, card.campaign()).unwrap(); // exactly we created campaign types inside cards

        if matches!(move_type, MoveType::ContinueRoute) {
            // continuing route
            let route = match player {
                Player::Player1 => &mut campaign.player_1_route,
                Player::Player2 => &mut campaign.player_2_route,
            };

            if let Some(last_card) = route.last() {
                if let CardType::Rank(last_rank) = last_card.card_type() {
                    if !matches!(card.card_type(), CardType::Rank(rank) if rank > last_rank) {
                        // if last card in the route is ranked and has greater rank than the chosen card's one
                        return Err(GameError::LastCardHasGreaterRank);
                    }
                }
            }

            // appending the card to the route
            route.push(Self::take_card_by_ref(players_hand, card)?)
        } else {
            // making card free

            if matches!(take_card_from, TakeCardFrom::FreeCards(campaign_type) if campaign_type == campaign.campaign_type)
            {
                // if player trying to put and take the same card
                return Err(GameError::PuttingAndTakingSameCard);
            }

            // appending the card to the free cards
            campaign
                .free_cards
                .push(Self::take_card_by_ref(players_hand, card)?)
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

        Ok(())
    }

    #[inline]
    fn take_card_by_ref(players_hand: &mut Vec<Card>, card: &Card) -> GameResult<Card> {
        let card = players_hand.swap_remove(
            players_hand
                .iter()
                .position(|players_card| *players_card == *card)
                .ok_or(GameError::PlayerDoesNotHaveSuchCard)?,
        );
        Ok(card)
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

        GameInfo {
            campaigns,
            main_deck_len,
            players_hand,
            is_game_ended,
        }
    }

    pub fn game_results(&self) -> GameInfoResults {
        let (mut player_1_points, mut player_2_points) = (0, 0);

        for campaign in self.playground.campaigns.iter() {
            for (route, players_points) in [
                (&campaign.player_1_route, &mut player_1_points),
                (&campaign.player_2_route, &mut player_2_points),
            ] {
                let (points, multiplier) =
                    route.iter().fold((0, 1), |(points, multiplier), card| {
                        match card.card_type() {
                            CardType::HandShake => (points, multiplier + 1),
                            CardType::Rank(rank) => (points + *rank as usize, multiplier),
                        }
                    });

                // calculating route income
                *players_points += points * multiplier - self.campaign_outcome;
            }
        }

        GameInfoResults {
            player_1_points,
            player_2_points,
        }
    }
}
