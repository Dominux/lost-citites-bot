<script lang="ts">
	import Card from './Card.svelte'

	const playersHand = [0, 1, 2, 3, 4, 5, 6, 7]
</script>

<div class="players-hand">
	{#each playersHand as _card}
		<Card />
	{/each}
</div>

<style scoped>
	.players-hand {
		display: inline-flex;
		justify-content: space-around;
	}
</style>
