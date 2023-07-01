import init, { Config, GameInfo, Game as InnerGame } from '../../pkg'
import { GameBuilder } from '../../pkg/lost_cities_game_lib'

import { PutTo, TakeFrom } from './move_process'

await init()

const CONFIG = new Config(5, 9, 3, 8, 20, 8, 20)

export type Player = 'Player1' | 'Player2'

export default class Game {
	private inner: InnerGame
	private player_1_info: GameInfo
	private player_2_info: GameInfo

	constructor() {
		this.inner = GameBuilder.build(CONFIG)

		// caching
		this.update_info('Player1')
		this.update_info('Player2')
	}

	makeMove(
		player: Player,
		card_id: number,
		move_type: PutTo,
		take_from: TakeFrom,
		free_cards_campaign: number | null = null
	) {
		const is_continue_route = move_type == PutTo.Route
		const is_take_from_main_deck = take_from == TakeFrom.MainDeck

		this.inner.make_move(
			player,
			card_id,
			is_continue_route,
			is_take_from_main_deck,
			free_cards_campaign
		)

		// caching
		this.update_info('Player1')
		this.update_info('Player2')
	}

	private update_info(player: Player) {
		if (player == 'Player1') {
			this.player_1_info = this.inner.game_info(player)
		} else {
			this.player_2_info = this.inner.game_info(player)
		}
	}

	get_info(player: Player): GameInfo {
		if (player == 'Player1') {
			return this.player_1_info
		} else {
			return this.player_2_info
		}
	}
}
