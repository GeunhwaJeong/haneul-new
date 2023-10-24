// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { useActiveAccount } from '_app/hooks/useActiveAccount';
import { Coins, useBalanceConversion, useCoinsReFetchingConfig } from '_hooks';
import { HANEUL_CONVERSION_RATE } from '_pages/swap/constants';
import { useFormatCoin } from '@haneullabs/core';
import { useHaneulClientQuery } from '@haneullabs/dapp-kit';
import BigNumber from 'bignumber.js';

export function useSwapData({
	baseCoinType,
	quoteCoinType,
	activeCoinType,
}: {
	baseCoinType: string;
	quoteCoinType: string;
	activeCoinType: string;
}) {
	const activeAccount = useActiveAccount();
	const activeAccountAddress = activeAccount?.address;
	const { staleTime, refetchInterval } = useCoinsReFetchingConfig();

	const { data: baseCoinBalanceData, isPending: baseCoinBalanceDataLoading } = useHaneulClientQuery(
		'getBalance',
		{ coinType: baseCoinType, owner: activeAccountAddress! },
		{ enabled: !!activeAccountAddress, refetchInterval, staleTime },
	);

	const { data: quoteCoinBalanceData, isPending: quoteCoinBalanceDataLoading } = useHaneulClientQuery(
		'getBalance',
		{ coinType: quoteCoinType, owner: activeAccountAddress! },
		{ enabled: !!activeAccountAddress, refetchInterval, staleTime },
	);

	const rawBaseBalance = baseCoinBalanceData?.totalBalance;
	const rawQuoteBalance = quoteCoinBalanceData?.totalBalance;

	const [formattedBaseBalance, baseCoinSymbol, baseCoinMetadata] = useFormatCoin(
		rawBaseBalance,
		baseCoinType,
	);
	const [formattedQuoteBalance, quoteCoinSymbol, quoteCoinMetadata] = useFormatCoin(
		rawQuoteBalance,
		quoteCoinType,
	);

	return {
		baseCoinBalanceData,
		quoteCoinBalanceData,
		formattedBaseBalance,
		formattedQuoteBalance,
		baseCoinSymbol,
		quoteCoinSymbol,
		baseCoinMetadata,
		quoteCoinMetadata,
		isPending: baseCoinBalanceDataLoading || quoteCoinBalanceDataLoading,
	};
}

export function useHaneulUsdcBalanceConversion({ amount }: { amount: string }) {
	const haneulUsdc = useBalanceConversion(
		new BigNumber(amount),
		Coins.HANEUL,
		Coins.USDC,
		-HANEUL_CONVERSION_RATE,
	);

	const usdcHaneul = useBalanceConversion(
		new BigNumber(amount),
		Coins.USDC,
		Coins.HANEUL,
		HANEUL_CONVERSION_RATE,
	);

	return {
		haneulUsdc,
		usdcHaneul,
	};
}
