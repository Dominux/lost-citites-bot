<script lang="ts">
	import { Card as CardModel } from '../../pkg'
	import Card from './Card.svelte'

	export let route: Array<CardModel>
	export let campaign_type: number
</script>

<div class="card-sizes card-border-radius route">
	<!-- {#each cards as card (card.id)}
		<Card {card} />
	{:else} -->
	{campaign_type}
	<!-- {/each} -->
</div>

<style scoped>
	.route {
		margin: 2vw;
		text-align: center;
		padding-top: 2rem;
		box-shadow: 0 0 0 2px blue;
	}
</style>
