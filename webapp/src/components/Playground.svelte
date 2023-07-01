<script>
	import { MoveProcess, MoveStage } from '../entities/move_process'
	import { gameStore } from '../stores/game'
	import { moveProcessStore } from '../stores/move_process'
	import Campaign from './Campaign.svelte'
	import MainDeck from './MainDeck.svelte'
	import PlayersHand from './PlayersHand.svelte'

	$: if ($moveProcessStore.stage == MoveStage.SubmitingMove) {
		// submiting move
		gameStore.update((game) => {
			game.makeMove(
				$moveProcessStore.player,
				$moveProcessStore.card.id,
				$moveProcessStore.putTo,
				$moveProcessStore.takeFrom,
				// @ts-ignore
				$moveProcessStore.freeCardsCampaign
			)

			return game
		})

		// updating moveProcess
		const nextPlayerTurn =
			$moveProcessStore.player == 'Player1' ? 'Player2' : 'Player1'
		const mp = new MoveProcess(nextPlayerTurn)
		moveProcessStore.set(mp)
	}
</script>

<div class="playground_wrapper">
	<div class="playground">
		<div class="campaigns">
			{#each [...$gameStore
					.get_info($moveProcessStore.player)
					.campaigns.entries()] as [campaign_type, campaign] (campaign_type)}
				<Campaign {campaign} {campaign_type} />
			{/each}
		</div>

		<MainDeck />
	</div>

	<PlayersHand />
</div>

<style scoped>
	.playground_wrapper {
		display: flex;
		flex-direction: column;
	}

	.playground {
		display: inline-flex;
		justify-content: space-evenly;
		width: 100%;
	}

	.campaigns {
		display: inline-flex;
		justify-content: center;
	}
</style>
