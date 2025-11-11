import type { Card as CardModel } from '../pkg'

const CAMPAIGN_ROUTES_COLORS = ['yellow', 'blue', 'white', 'green', 'red']

export const getCardClass = (campaignType) =>
	`bg-${CAMPAIGN_ROUTES_COLORS[campaignType]}-route`
