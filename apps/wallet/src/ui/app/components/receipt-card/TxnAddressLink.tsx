// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { formatAddress } from '@haneullabs/haneul.js';

import ExplorerLink from '_components/explorer-link';
import { ExplorerLinkType } from '_components/explorer-link/ExplorerLinkType';

type TxnAddressLinkProps = {
    address: string;
};

export function TxnAddressLink({ address }: TxnAddressLinkProps) {
    return (
        <ExplorerLink
            type={ExplorerLinkType.address}
            address={address}
            title="View on Haneul Explorer"
            className="text-haneul-dark font-mono text-body font-semibold no-underline tracking-wider"
            showIcon={false}
        >
            {formatAddress(address)}
        </ExplorerLink>
    );
}
