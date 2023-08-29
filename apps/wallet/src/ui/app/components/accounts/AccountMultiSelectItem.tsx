// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useResolveHaneulNSName } from '@haneullabs/core';
import { CheckFill16 } from '@haneullabs/icons';

import { formatAddress } from '@haneullabs/haneul.js/utils';
import * as ToggleGroup from '@radix-ui/react-toggle-group';
import cn from 'classnames';
import { AccountItem } from './AccountItem';
import { type SerializedUIAccount } from '_src/background/accounts/Account';

type AccountMultiSelectItemProps = {
	account: SerializedUIAccount;
	state?: 'selected' | 'disabled';
};

export function AccountMultiSelectItem({ account, state }: AccountMultiSelectItemProps) {
	const { data: domainName } = useResolveHaneulNSName(account.address);
	return (
		<ToggleGroup.Item asChild value={account.id}>
			<AccountItem
				name={account.nickname ?? domainName ?? formatAddress(account.address)}
				/* todo: implement account icon */
				address={account.address}
				disabled={state === 'disabled'}
				selected={state === 'selected'}
				after={
					<div
						className={cn(`flex items-center justify-center ml-auto text-hero/10`, {
							'text-success': state === 'selected',
						})}
					>
						<CheckFill16 className={cn('h-4 w-4', { 'opacity-50': state === 'disabled' })} />
					</div>
				}
			/>
		</ToggleGroup.Item>
	);
}
