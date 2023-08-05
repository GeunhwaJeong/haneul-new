// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useFormatCoin } from '@haneullabs/core';
import { type CoinStruct } from '@haneullabs/haneul.js/client';
import { Text } from '@haneullabs/ui';

import { ObjectLink } from '~/ui/InternalLink';

type CoinItemProps = {
	coin: CoinStruct;
};

export default function CoinItem({ coin }: CoinItemProps) {
	const [formattedBalance, symbol] = useFormatCoin(coin.balance, coin.coinType);
	return (
		<div className="flex min-w-50 max-w-80 items-center justify-between rounded-lg bg-white px-3 py-2 shadow-panel">
			<ObjectLink objectId={coin.coinObjectId} />
			<div className="col-span-3 inline-flex items-center gap-1">
				<Text color="steel-darker" variant="bodySmall/medium">
					{formattedBalance}
				</Text>
				<Text color="steel" variant="subtitleSmallExtra/normal">
					{symbol}
				</Text>
			</div>
		</div>
	);
}
