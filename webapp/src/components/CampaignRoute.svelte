<script lang="ts">
	import { Card as CardModel } from '../../pkg'
	import { PutTo } from '../entities/move_process'
	import { moveProcessStore } from '../stores/move_process'

	export let isAvailableToPutCard: boolean | null
	export let route: Array<CardModel>
	export let campaign_type: number

	const onSelect = () => {
		if (isAvailableToPutCard) {
			moveProcessStore.update((mp) => {
				mp.putCard(PutTo.Route)
				return mp
			})
		}
	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<div
	class="card-sizes card-border-radius route"
	class:available-route={isAvailableToPutCard}
	class:put-card={isAvailableToPutCard !== null &&
		$moveProcessStore.putTo == PutTo.Route &&
		campaign_type == $moveProcessStore.card.campaign}
	on:click={(_) => onSelect()}
>
	{campaign_type}
</div>

<style scoped>
	.route {
		margin: 2vw;
		text-align: center;
		padding-top: 2rem;
		box-shadow: 0 0 0 2px blue;
	}

	.available-route {
		box-shadow: 0 0 0 4px yellow;
	}

	.put-card {
		box-shadow: 0 0 0 4px orange;
	}
</style>
