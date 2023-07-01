<script lang="ts">
	import { moveProcessStore } from '../stores/move_process'
	import { MoveStage, TakeFrom } from '../entities/move_process'

	$: isAvailableToTakeCard =
		$moveProcessStore.stage == MoveStage.TakingNewCard

	const onSelect = () => {
		if (!isAvailableToTakeCard) return

		moveProcessStore.update((mp) => {
			mp.takeNewCard(TakeFrom.MainDeck)
			return mp
		})
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="card-sizes card-border-radius main-deck animated"
	class:available-to-take-card={isAvailableToTakeCard}
	on:click={(_) => onSelect()}
>
	Main Deck
</div>

<style>
	.main-deck {
		text-align: center;
		padding-top: 2rem;
		margin: 2vw;
		box-shadow: 0 0 0 2px rgba(225, 225, 225, 0.9),
			0 0 0 8px rgba(0, 0, 0, 0.9), 0 0 0 10px rgba(225, 225, 225, 0.9);
	}

	.available-to-take-card {
		box-shadow: 0 0 0 3px lightgreen, 0 0 0 8px rgba(0, 0, 0, 0.9),
			0 0 0 10px lightgreen;
	}
</style>
