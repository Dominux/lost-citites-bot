<script lang="ts">
	import { moveProcessStore } from '../stores/move_process'
	import { PutTo, TakeFrom } from '../entities/move_process'
	import type { Card as CardModel } from '../../pkg'
	import Card from './Card.svelte'

	export let campaign_type: number
	export let lastCard: CardModel
	export let isAvailableToPutCard = false
	export let isAvailableToTakeCard = false

	const campaignTypesColors = ['yellow', 'blue', 'white', 'green', 'red']

	const onSelect = () => {
		if (isAvailableToPutCard) {
			moveProcessStore.update((mp) => {
				mp.putCard(PutTo.FreeCards)
				return mp
			})
		} else if (isAvailableToTakeCard) {
			moveProcessStore.update((mp) => {
				mp.takeNewCard(TakeFrom.FreeCards, campaign_type)
				return mp
			})
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="card-sizes card-border-radius free-card-skeleton animated"
	class:available-free-cards={isAvailableToPutCard}
	class:put-card={$moveProcessStore.putTo == PutTo.FreeCards &&
		campaign_type == $moveProcessStore.card.campaign}
	class:available-to-take-card={isAvailableToTakeCard}
	on:click={(_) => onSelect()}
>
	{#if lastCard}
		<Card card={lastCard} />
	{:else}
		<div
			class="free-cards-cover card-border-radius"
			style={`background-color: ${campaignTypesColors[campaign_type]}`}
		/>
	{/if}
</div>

<style scoped>
	.free-card-skeleton {
		text-align: center;
		padding-top: 2rem;
		margin: 2vw;
		box-shadow: 0 0 0 2px rgba(225, 225, 225, 0.9),
			0 0 0 8px rgba(0, 0, 0, 0.9), 0 0 0 10px rgba(225, 225, 225, 0.9);
	}

	.available-free-cards {
		box-shadow: 0 0 0 3px yellow, 0 0 0 8px rgba(0, 0, 0, 0.9),
			0 0 0 10px yellow;
	}

	.put-card {
		box-shadow: 0 0 0 3px orange, 0 0 0 8px rgba(0, 0, 0, 0.9),
			0 0 0 10px orange;
	}

	.available-to-take-card {
		box-shadow: 0 0 0 3px lightgreen, 0 0 0 8px rgba(0, 0, 0, 0.9),
			0 0 0 10px lightgreen;
	}

	.free-cards-cover {
		min-width: 100%;
		min-height: 100%;
	}
</style>
