<script lang="ts">
	export let campaign_type: number
</script>

<div class="card-sizes card-border-radius free-card-skeleton">
	{campaign_type}
</div>

<style scoped>
	.free-card-skeleton {
		text-align: center;
		margin: 2vw;
		box-shadow: 0 0 0 2px rgba(225, 225, 225, 0.9),
			0 0 0 8px rgba(0, 0, 0, 0.9), 0 0 0 10px rgba(225, 225, 225, 0.9);
	}
</style>
