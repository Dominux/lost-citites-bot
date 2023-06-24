import { Config, Game as InnerGame } from '../../pkg'
import { GameBuilder } from '../../pkg/lost_cities_game_lib'

const CONFIG = new Config(5, 9, 3, 8, 20, 8, 20)

export default class Game {
	private inner: InnerGame

	constructor() {
		this.inner = GameBuilder.build(CONFIG)
	}
}
