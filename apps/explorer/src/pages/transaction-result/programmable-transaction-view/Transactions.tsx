// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { type HaneulTransaction } from '@haneullabs/haneul.js';

import { Transaction } from './Transaction';

import { TableHeader } from '~/ui/TableHeader';

interface Props {
    transactions: HaneulTransaction[];
}

export function Transactions({ transactions }: Props) {
    if (!transactions?.length) {
        return null;
    }

    return (
        <>
            <TableHeader>Transactions</TableHeader>
            <ul className="flex flex-col gap-8">
                {transactions.map((transaction, index) => {
                    const [[type, data]] = Object.entries(transaction);

                    return (
                        <li key={index}>
                            <Transaction type={type} data={data} />
                        </li>
                    );
                })}
            </ul>
        </>
    );
}
