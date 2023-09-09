<script lang="ts">
	import { moveProcessStore } from '../stores/move_process'
	import { Card as CardModel } from '../../pkg'
	import { PutTo } from '../entities/move_process'
	import Card from './Card.svelte'

	export let isPlayer: boolean
	export let isAvailableToPutCard: boolean | null
	export let route: Array<CardModel>
	export let campaign_type: number

	const onSelect = () => {
		if (!isAvailableToPutCard) return

		moveProcessStore.update((mp) => {
			mp.putCard(PutTo.Route)
			return mp
		})
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="card-sizes card-border-radius route animated"
	class:available-route={isAvailableToPutCard}
	class:put-card={isAvailableToPutCard !== null &&
		$moveProcessStore.putTo == PutTo.Route &&
		campaign_type == $moveProcessStore.card.campaign}
	class:foe-cards={!isPlayer}
	style="min-height: {7 + route.length * 4}vw;"
	on:click={(_) => onSelect()}
>
	{#each route as card, i (card.id)}
		<div class={i === 0 ? 'first-card' : 'next-card'}>
			<Card {card} />
		</div>
	{/each}
</div>

<style scoped>
	.route {
		margin: 2vw;
		padding-top: 2vw;
		box-shadow: 0 0 0 2px blue;
		/* min-height: fit-content; */
		/* min-height: 42vw; */
	}

	.available-route {
		box-shadow: 0 0 0 4px yellow;
	}

	.put-card {
		box-shadow: 0 0 0 4px orange;
	}

	.foe-cards {
		transform: rotate(180deg);
	}

	.first-card {
		margin-top: -2vw;
	}
	.next-card {
		margin-top: -14vw;
		position: relative;
	}
</style>
