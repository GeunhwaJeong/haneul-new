// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Heading } from '_src/ui/app/shared/heading';
import { Text } from '_src/ui/app/shared/text';
import { useFormatCoin, useHaneulCoinData } from '@haneullabs/core';
import { HANEUL_DECIMALS } from '@haneullabs/haneul.js/utils';
import BigNumber from 'bignumber.js';
import { useMemo } from 'react';

export type CoinProps = {
	type: string;
	amount: bigint;
};

export function CoinBalance({ amount: walletBalance, type }: CoinProps) {
	const [formatted, symbol] = useFormatCoin(walletBalance, type);
	const { data } = useHaneulCoinData();
	const { currentPrice } = data || {};

	const walletBalanceInUsd = useMemo(() => {
		if (!currentPrice) return null;
		const haneulPriceInUsd = new BigNumber(currentPrice);
		const walletBalanceInHaneul = new BigNumber(walletBalance.toString()).shiftedBy(-1 * HANEUL_DECIMALS);
		const value = walletBalanceInHaneul.multipliedBy(haneulPriceInUsd).toNumber();

		return `~${value.toLocaleString('en', {
			style: 'currency',
			currency: 'USD',
		})} USD`;
	}, [currentPrice, walletBalance]);

	return (
		<div className="flex flex-col gap-1 items-center justify-center">
			<div className="flex items-center justify-center gap-2">
				<Heading leading="none" variant="heading1" weight="bold" color="gray-90">
					{formatted}
				</Heading>

				<Heading variant="heading6" weight="medium" color="steel">
					{symbol}
				</Heading>
			</div>
			<div>
				{walletBalanceInUsd ? (
					<Text variant="caption" weight="medium" color="steel">
						{walletBalanceInUsd}
					</Text>
				) : null}
			</div>
		</div>
	);
}
