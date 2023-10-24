// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { Coins, getUSDCurrency, useBalanceConversion } from '_app/hooks/useDeepBook';
import { Text } from '_app/shared/text';
import { DescriptionItem } from '_pages/approval-request/transaction-request/DescriptionList';
import { HANEUL_CONVERSION_RATE, WALLET_FEES_PERCENTAGE } from '_pages/swap/constants';
import { GAS_TYPE_ARG } from '_redux/slices/haneul-objects/Coin';
import { useCoinMetadata, useFormatCoin } from '@haneullabs/core';
import { HANEUL_TYPE_ARG } from '@haneullabs/haneul.js/utils';
import BigNumber from 'bignumber.js';
import { useMemo } from 'react';

export function GasFeeSection({
	activeCoinType,
	totalGas,
	amount,
	isValid,
}: {
	activeCoinType: string | null;
	amount: string;
	isValid: boolean;
	totalGas: string;
}) {
	const { data: activeCoinData } = useCoinMetadata(activeCoinType);
	const isAsk = activeCoinType === HANEUL_TYPE_ARG;

	const estimatedFees = useMemo(() => {
		if (!amount || !isValid) {
			return null;
		}

		return new BigNumber(amount).times(WALLET_FEES_PERCENTAGE / 100);
	}, [amount, isValid]);

	const { rawValue } = useBalanceConversion(
		estimatedFees,
		isAsk ? Coins.HANEUL : Coins.USDC,
		isAsk ? Coins.USDC : Coins.HANEUL,
		isAsk ? -HANEUL_CONVERSION_RATE : HANEUL_CONVERSION_RATE,
	);

	const [gas, symbol] = useFormatCoin(totalGas, GAS_TYPE_ARG);

	const formattedEstimatedFees = getUSDCurrency(rawValue);

	return (
		<div className="flex flex-col border border-hero-darkest/20 rounded-xl p-5 gap-4 border-solid">
			<DescriptionItem
				title={
					<Text variant="bodySmall" weight="medium" color="steel-dark">
						Fees ({WALLET_FEES_PERCENTAGE}%)
					</Text>
				}
			>
				<Text variant="bodySmall" weight="medium" color="steel-darker">
					{estimatedFees
						? `${estimatedFees.toLocaleString()} ${activeCoinData?.symbol} (${formattedEstimatedFees})`
						: '--'}
				</Text>
			</DescriptionItem>

			<div className="bg-gray-40 h-px w-full" />

			<DescriptionItem
				title={
					<Text variant="bodySmall" weight="medium" color="steel-dark">
						Estimated Gas Fee
					</Text>
				}
			>
				<Text variant="bodySmall" weight="medium" color="steel-darker">
					{totalGas && isValid ? `${gas} ${symbol}` : '--'}
				</Text>
			</DescriptionItem>
		</div>
	);
}
