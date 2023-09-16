<script lang="ts">
	import type { Card as CardModel } from '../../pkg'

	export let card: CardModel

	const getCardSrc = (card: CardModel) => {
		const cardPrefix = card.campaign + 1
		const cardPostfix = card.card_type.Rank
			? card.card_type.Rank
			: 'handshake'
		return `./assets/card_images/${cardPrefix}_${cardPostfix}.webp`
	}
</script>

<div class="card card-sizes card-border-radius">
	<h5 class="card-title">
		{card.card_type.Rank || '🤝'}
	</h5>
	<img class="img card-border-radius" src={getCardSrc(card)} alt="Card" />
	<!-- Campaign: {card.campaign}
	<br />
	Rank: {typeof card.card_type == 'string'
		? card.card_type
		: card.card_type.Rank} -->
</div>

<style scoped>
	.card {
		text-align: center;
		box-shadow: 0 0 0 2px blanchedalmond;
		background-color: black;
	}

	.card-title {
		padding: 0.2vw;
	}

	.img {
		max-width: 100%;
		max-height: 100%;
	}
</style>
