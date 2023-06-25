<script lang="ts">
	import { gameStore } from '../stores/game'
	import { dndzone } from 'svelte-dnd-action'
	import { flip } from 'svelte/animate'
	import Card from './Card.svelte'

	$: dndOptions = {
		items: $gameStore.get_info('Player1').players_hand,
	}
</script>

<div
	class="players-hand"
	use:dndzone={dndOptions}
	on:consider={(e) => console.log('lmao')}
>
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
