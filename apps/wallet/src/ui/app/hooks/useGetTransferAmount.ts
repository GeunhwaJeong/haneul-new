// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getAmount } from '_helpers';
import type { HaneulTransactionBlockResponse } from '@haneullabs/haneul.js/client';
import { HANEUL_TYPE_ARG } from '@haneullabs/haneul.js/utils';
import { useMemo } from 'react';

export function useGetTransferAmount({
	txn,
	activeAddress,
}: {
	txn: HaneulTransactionBlockResponse;
	activeAddress: string;
}) {
	const { effects, events } = txn;
	// const { coins } = getEventsSummary(events!, activeAddress);

	const haneulTransfer = useMemo(() => {
		const txdetails = txn.transaction?.data.transaction!;
		return getAmount(txdetails, effects!, events!)?.map(
			({ amount, coinType, recipientAddress }) => {
				return {
					amount: amount || 0,
					coinType: coinType || HANEUL_TYPE_ARG,
					receiverAddress: recipientAddress,
				};
			},
		);
	}, [txn, effects, events]);

	// MUSTFIX(chris)
	// const transferAmount = useMemo(() => {
	//     return haneulTransfer?.length
	//         ? haneulTransfer
	//         : coins.filter(
	//               ({ receiverAddress }) => receiverAddress === activeAddress
	//           );
	// }, [haneulTransfer, coins, activeAddress]);

	// return haneulTransfer ?? transferAmount;
	return haneulTransfer;
}
