<script lang="ts">
	import { GameInfoCampaign } from '../../pkg'
	import FreeCardsDeck from './FreeCardsDeck.svelte'
	import CampaignRoute from './CampaignRoute.svelte'

	export let campaign: GameInfoCampaign
	export let campaign_type: number
</script>

<div>
	<CampaignRoute route={campaign.foes_route} {campaign_type} />
	<FreeCardsDeck {campaign_type} />
	<CampaignRoute route={campaign.players_route} {campaign_type} />
</div>
