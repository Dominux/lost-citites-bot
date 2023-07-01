<script>
	import { gameStore } from '../stores/game'
	import Campaign from './Campaign.svelte'
	import PlayersHand from './PlayersHand.svelte'
</script>

<div class="playground">
	<div class="campaigns">
		{#each [...$gameStore
				.get_info('Player1')
				.campaigns.entries()] as [campaign_type, campaign] (campaign_type)}
			<Campaign {campaign} {campaign_type} />
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
