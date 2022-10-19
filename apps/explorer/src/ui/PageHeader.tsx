// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type TransactionKindName } from '@haneullabs/haneul.js';
import toast from 'react-hot-toast';

import { Badge } from './Badge';
import { Heading } from './Heading';
import { ReactComponent as CopyIcon } from './icons/copy.svg';
import { ReactComponent as ImageIcon } from './icons/image.svg';
import { ReactComponent as SenderIcon } from './icons/sender.svg';
import { ReactComponent as CallIcon } from './icons/transactions/call.svg';
import { ReactComponent as ChangeEpochIcon } from './icons/transactions/changeEpoch.svg';
import { ReactComponent as PayIcon } from './icons/transactions/pay.svg';
import { ReactComponent as PublishIcon } from './icons/transactions/publish.svg';
import { ReactComponent as TransferObjectIcon } from './icons/transactions/transferObject.svg';
import { ReactComponent as TransferHaneulIcon } from './icons/transactions/transferHaneul.svg';

export type PageHeaderType =
    | TransactionKindName
    | 'Address'
    | 'Object'
    | 'Package';

export interface PageHeaderProps {
    title: string;
    type: PageHeaderType;
    status?: 'success' | 'failure';
}

const TYPE_TO_ICON: Record<PageHeaderType, typeof CallIcon> = {
    Call: CallIcon,
    ChangeEpoch: ChangeEpochIcon,
    Pay: PayIcon,
    Publish: PublishIcon,
    TransferObject: TransferObjectIcon,
    TransferHaneul: TransferHaneulIcon,
    Object: ImageIcon,
    Package: CallIcon,
    Address: () => (
        <SenderIcon
            style={{
                '--icon-primary-color': 'var(--haneul-steel)',
                '--icon-secondary-color': 'white',
            }}
        />
    ),
};

const STATUS_TO_TEXT = {
    success: 'Success',
    failure: 'Failure',
};

export function PageHeader({ title, type, status }: PageHeaderProps) {
    const Icon = TYPE_TO_ICON[type];
    return (
        <div className="flex flex-col gap-3">
            <div className="text-haneul-grey-85 flex items-center gap-2">
                <Icon className="text-haneul-steel" />
                <Heading variant="heading4" weight="semibold">
                    {type}
                </Heading>
            </div>
            <div className="flex flex-col lg:flex-row gap-2">
                <div className="flex items-center gap-2 min-w-0">
                    <div className="break-words min-w-0">
                        <Heading as="h2" variant="heading2" weight="bold" mono>
                            {title}
                        </Heading>
                    </div>
                    <button
                        onClick={() => {
                            navigator.clipboard.writeText(title);
                            toast.success('Copied!');
                        }}
                        className="bg-transparent border-none cursor-pointer p-0 m-0 text-haneul-steel flex justify-center items-center"
                    >
                        <CopyIcon />
                    </button>
                </div>

                {status && (
                    <div>
                        <Badge variant={status}>{STATUS_TO_TEXT[status]}</Badge>
                    </div>
                )}
            </div>
        </div>
    );
}
