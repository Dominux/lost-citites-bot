<script lang="ts">
	import { gameStore } from '../stores/game'
	import { moveProcessStore } from '../stores/move_process'

	import type { Card as CardModel } from '../../pkg'

	import Card from './Card.svelte'
	import { MoveStage } from '../entities/move_process'

	let cards = $gameStore.get_info('Player1').players_hand

	const onSelect = (card: CardModel) => {
		if (
			![MoveStage.SelectingCard, MoveStage.PutingCard].includes(
				$moveProcessStore.stage
			)
		)
			return

		moveProcessStore.update((mp) => {
			mp.selectCard(card)
			return mp
		})
	}

	const onDiselect = () => {
		if ($moveProcessStore.stage != MoveStage.PutingCard) return

		moveProcessStore.update((mp) => {
			mp.diselectCard()
			return mp
		})
	}
</script>

<div class="players-hand">
	{#each cards as card (card.id)}
		<!-- svelte-ignore a11y-click-events-have-key-events -->
		<span
			class="card-border-radius animated"
			class:selected_card={card.id == $moveProcessStore.card?.id}
			on:click={(_) => onSelect(card)}
			on:blur={(_) => onDiselect()}
		>
			<Card {card} />
		</span>
	{/each}
</div>

<style scoped>
	.players-hand {
		display: inline-flex;
		justify-content: space-around;
	}

	.selected_card {
		transform: translateY(calc(var(--card-width) * -0.2));
	}
</style>
