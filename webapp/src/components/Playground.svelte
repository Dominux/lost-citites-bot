<script>
	import Campaign from './Campaign.svelte'

	const campaigns = [1, 2, 3, 4, 5]
</script>

<div class="playground">
	{#each campaigns as campaign}
		<Campaign />
	{/each}
</div>

<style>
	.playground {
		display: inline-flex;
	}
</style>
