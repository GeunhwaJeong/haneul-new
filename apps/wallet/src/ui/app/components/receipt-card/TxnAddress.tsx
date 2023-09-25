// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Text } from '_src/ui/app/shared/text';
import { useResolveHaneulNSName } from '@haneullabs/core';

import { TxnAddressLink } from './TxnAddressLink';

type TxnAddressProps = {
	address: string;
	label: string;
};

export function TxnAddress({ address, label }: TxnAddressProps) {
	const { data: domainName } = useResolveHaneulNSName(address);

	return (
		<div className="flex justify-between w-full items-center py-3.5 first:pt-0">
			<Text variant="body" weight="medium" color="steel-darker">
				{label}
			</Text>
			<div className="flex gap-1 items-center">
				<TxnAddressLink address={domainName ?? address} />
			</div>
		</div>
	);
}
