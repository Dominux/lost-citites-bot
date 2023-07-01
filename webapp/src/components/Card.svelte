<script lang="ts">
	import { createEventDispatcher } from 'svelte'

	import type { Card as CardModel } from '../../pkg'

	export let card: CardModel

	let isSelected = false

	const dispatch = createEventDispatcher()

	const click = () => {
		isSelected = true
		dispatch('select')
	}

	const diselect = () => {
		isSelected = false
		dispatch('diselect')
	}
</script>

<button
	class="card card-sizes card-border-radius"
	on:click={click}
	on:blur={diselect}
>
	Campaign: {card.campaign}
	<br />
	Rank: {typeof card.card_type == 'string'
		? card.card_type
		: card.card_type.Rank}
</button>

<style scoped>
	.card {
		text-align: center;
		padding-top: 2rem;
		box-shadow: 0 0 0 2px red;
	}

	.card:focus-within {
		box-shadow: 0 0 0 2px yellow;
	}
</style>
