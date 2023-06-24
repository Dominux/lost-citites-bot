<script lang="ts">
	export let campaign_type: number
</script>

<div class="card-sizes card-border-radius route">{campaign_type}</div>

<style scoped>
	.route {
		margin: 2vw;
		text-align: center;
		box-shadow: 0 0 0 2px blue;
	}
</style>
