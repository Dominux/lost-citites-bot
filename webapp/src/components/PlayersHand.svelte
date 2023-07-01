<script lang="ts">
	import { gameStore } from '../stores/game'
	import Card from './Card.svelte'

	let cards = $gameStore.get_info('Player1').players_hand

	let selectedCardId: number | null = null

	const onDiselect = (cardId: number) => {
		if (selectedCardId == cardId) {
			selectedCardId = null
		}
	}
</script>

<div class="players-hand">
	{#each cards as card (card.id)}
		<Card
			{card}
			on:select={() => {
				selectedCardId = card.id
			}}
			on:diselect={() => onDiselect(card.id)}
		/>
	{/each}
</div>

<style scoped>
	.players-hand {
		display: inline-flex;
		justify-content: space-around;
	}
</style>
