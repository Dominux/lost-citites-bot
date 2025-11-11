<script lang="ts">
	import type { Card as CardModel } from '../../pkg'
	import { getCardClass } from '../common'

	export let card: CardModel
</script>

<div class="iphone-mockup card card-sizes card-border-radius">
	<div class="speaker-notch">
		<h4 style="font-weight: bold;">
			{card.card_type.Rank || '🤝'}
		</h4>
	</div>
	<div class="screen card-border-radius {getCardClass(card.campaign)}"></div>
</div>

<style scoped>
	.card {
		text-align: center;
		box-shadow: 0 0 0 2px blanchedalmond;
		background-color: black;
		box-sizing: border-box;
	}

	.iphone-mockup {
		/* Main body of the phone */
		position: relative;
		background-color: #000;
		padding: 5%; /* Padding for the screen inside the frame */
		box-sizing: border-box;
	}

	.screen {
		/* The actual display area */
		width: 100%;
		height: 100%;
		color: #333;
		text-align: center;
		overflow: hidden; /* Ensures content stays within the rounded corners */
	}

	.speaker-notch {
		/* The notch at the top */
		position: absolute;
		left: 50%;
		transform: translateX(-50%);
		min-width: 60%;
		height: fit-content;
		background-color: #000;
		border-radius: 0 0 40% 40%;
		z-index: 10;
		display: flex;
		justify-content: center;
	}
</style>
