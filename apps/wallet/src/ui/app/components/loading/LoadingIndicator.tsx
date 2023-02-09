// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Spinner16 } from '@haneullabs/icons';
import { cva, type VariantProps } from 'class-variance-authority';

const styles = cva('', {
    variants: {
        color: {
            inherit: 'text-inherit',
            haneul: 'text-haneul',
        },
    },
});

export type LoadingIndicatorProps = VariantProps<typeof styles>;

const LoadingIndicator = ({ color = 'haneul' }: LoadingIndicatorProps) => {
    return (
        <Spinner16 className={styles({ className: 'animate-spin', color })} />
    );
};

export default LoadingIndicator;
