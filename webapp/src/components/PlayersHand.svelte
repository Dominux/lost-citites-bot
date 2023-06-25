<script lang="ts">
	import { gameStore } from '../stores/game'
	import Card from './Card.svelte'
</script>

<div class="players-hand">
	{#each $gameStore.get_info('Player1').players_hand as card}
		<Card {card} />
	{/each}
</div>

<style scoped>
	.players-hand {
		display: inline-flex;
		justify-content: space-around;
	}
</style>
