<script lang="ts">
	import FreeCardsDeck from './FreeCardsDeck.svelte'
	import CampaignRoute from './CampaignRoute.svelte'

	export let campaign_type: number
</script>

<div>
	<CampaignRoute {campaign_type} />
	<FreeCardsDeck {campaign_type} />
	<CampaignRoute {campaign_type} />
</div>
