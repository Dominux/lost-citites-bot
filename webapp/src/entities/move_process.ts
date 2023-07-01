import type { Card } from '../../pkg/lost_cities_game_lib'

export enum MoveStage {
	SelectingCard,
	PutingCard,
	TakingNewCard,
}

export enum PutTo {
	Route,
	FreeCards,
}

export enum TakeFrom {
	MainDeck,
	FreeCards,
}

export class MoveProcess {
	protected _player: 'Player1' | 'Player2'
	protected _stage: MoveStage
	protected _selectedCard: Card
	protected _putTo: PutTo

	get stage(): MoveStage {
		return this._stage
	}

	get card(): Card {
		return this._selectedCard
	}

	get putTo(): PutTo {
		return this._putTo
	}

	constructor() {
		this._stage = MoveStage.SelectingCard
	}

	selectCard(card: Card) {
		this._selectedCard = card
		this._stage = MoveStage.PutingCard
	}

	diselectCard() {
		this._selectedCard = null
		this._stage = MoveStage.SelectingCard
	}

	putCard(putTo: PutTo) {
		this._putTo = putTo
		this._stage = MoveStage.TakingNewCard
	}

	takeNewCard(takeFrom: TakeFrom, freeCardsCampaign: number | null) {}
}
