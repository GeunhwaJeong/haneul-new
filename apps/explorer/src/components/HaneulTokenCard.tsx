// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { COIN_GECKO_HANEUL_URL, useHaneulCoinData } from '@haneullabs/core';
import { Haneul } from '@haneullabs/icons';
import { Text } from '@haneullabs/ui';

import { Card } from '~/ui/Card';
import { ButtonOrLink } from '~/ui/utils/ButtonOrLink';

export function HaneulTokenCard() {
	const { data } = useHaneulCoinData();
	const { currentPrice } = data || {};

	const formattedPrice = currentPrice
		? currentPrice.toLocaleString('en', {
				style: 'currency',
				currency: 'USD',
		  })
		: '--';

	return (
		<ButtonOrLink href={COIN_GECKO_HANEUL_URL}>
			<Card growOnHover bg="white/80" spacing="lg" height="full">
				<div className="flex items-center gap-2">
					<div className="h-5 w-5 flex-shrink-0 rounded-full bg-haneul p-1">
						<Haneul className="h-full w-full text-white" />
					</div>
					<div className="flex w-full flex-col gap-0.5">
						<Text variant="body/semibold" color="steel-darker">
							1 HANEUL = {formattedPrice}
						</Text>
						<Text variant="subtitleSmallExtra/medium" color="steel">
							via CoinGecko
						</Text>
					</div>
				</div>
			</Card>
		</ButtonOrLink>
	);
}
