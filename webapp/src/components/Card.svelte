<script lang="ts">
	import type { Card as CardModel } from '../../pkg'

	export let card: CardModel
</script>

<div class="card card-sizes card-border-radius">
	Campaign: {card.campaign}
	<br />
	Rank: {typeof card.card_type == 'string'
		? card.card_type
		: card.card_type.Rank}
</div>

<style scoped>
	.card {
		text-align: center;
		padding-top: 2rem;
		box-shadow: 0 0 0 2px red;
	}
</style>
