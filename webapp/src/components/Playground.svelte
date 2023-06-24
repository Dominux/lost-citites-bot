<script>
	import Campaign from './Campaign.svelte'
	import PlayersHand from './PlayersHand.svelte'

	const campaigns = [1, 2, 3, 4, 5]
</script>

<div class="playground">
	<div class="campaigns">
		{#each campaigns as campaign}
			<Campaign />
		{/each}
	</div>
	<PlayersHand />
</div>

<style scoped>
	.playground {
		display: flex;
		flex-direction: column;
	}

	.campaigns {
		display: inline-flex;
		width: 100%;
		justify-content: center;
	}
</style>
