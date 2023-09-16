<script lang="ts">
	import { moveProcessStore } from '../stores/move_process'
	import { GameInfoCampaign } from '../../pkg'

	import FreeCardsDeck from './FreeCardsDeck.svelte'
	import CampaignRoute from './CampaignRoute.svelte'
	import { MoveStage, PutTo } from '../entities/move_process'

	export let campaign: GameInfoCampaign
	export let campaign_type: number

	let canPutCardToRoute = false

	$: isAvailableToPutCard =
		$moveProcessStore.stage == MoveStage.PutingCard &&
		campaign_type == $moveProcessStore.card.campaign

	$: isAvailableToTakeCard =
		$moveProcessStore.stage == MoveStage.TakingNewCard &&
		campaign.last_free_card &&
		!(
			campaign_type == $moveProcessStore.card.campaign &&
			$moveProcessStore.putTo == PutTo.FreeCards
		)

	$: if ($moveProcessStore.card?.campaign == campaign_type) {
		const lastCardType = campaign.players_route.slice(-1)[0]?.card_type
		const selectedCardType = $moveProcessStore.card.card_type

		if (lastCardType?.Rank) {
			canPutCardToRoute = selectedCardType.Rank > lastCardType.Rank
		} else {
			canPutCardToRoute = true
		}
	}
</script>

<div>
	<CampaignRoute
		isPlayer={false}
		isAvailableToPutCard={null}
		route={campaign.foes_route}
		{campaign_type}
	/>
	<FreeCardsDeck
		{campaign_type}
		lastCard={campaign.last_free_card}
		{isAvailableToPutCard}
		{isAvailableToTakeCard}
	/>
	<CampaignRoute
		isPlayer={true}
		isAvailableToPutCard={isAvailableToPutCard && canPutCardToRoute}
		route={campaign.players_route}
		{campaign_type}
	/>
</div>
