import type { Card } from '../../pkg/lost_cities_game_lib'
import type { Player } from './game'

export enum MoveStage {
	SelectingCard,
	PutingCard,
	TakingNewCard,
	SubmitingMove,
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
	protected _stage: MoveStage
	protected _selectedCard: Card
	protected _putTo: PutTo
	protected _takeFrom: TakeFrom
	protected _freeCardsCampaign?: number

	get stage(): MoveStage {
		return this._stage
	}

	get card(): Card {
		return this._selectedCard
	}

	get putTo(): PutTo {
		return this._putTo
	}

	get takeFrom(): TakeFrom {
		return this._takeFrom
	}

	get freeCardsCampaign(): number | void {
		return this._freeCardsCampaign
	}

	constructor(readonly player: Player) {
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

	takeNewCard(takeFrom: TakeFrom, freeCardsCampaign?: number) {
		this._takeFrom = takeFrom
		this._freeCardsCampaign = freeCardsCampaign
		this._stage = MoveStage.SubmitingMove
	}
}
