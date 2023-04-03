// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type MoveCallHaneulTransaction,
    type HaneulArgument,
    type HaneulMovePackage,
} from '@haneullabs/haneul.js';
import { type ReactNode } from 'react';

import { flattenHaneulArguments } from './utils';

import { ErrorBoundary } from '~/components/error-boundary/ErrorBoundary';
import { ObjectLink } from '~/ui/InternalLink';

export interface TransactionProps<T> {
    type: string;
    data: T;
}

function TransactionContent({
    type,
    children,
}: {
    type: string;
    children?: ReactNode;
}) {
    return (
        <>
            <div className="text-heading6 font-semibold text-steel-darker">
                {type}
            </div>
            {children && (
                <div className="text-bodyMedium pt-2 font-medium text-steel-dark">
                    {children}
                </div>
            )}
        </>
    );
}

function ArrayArgument({
    type,
    data,
}: TransactionProps<(HaneulArgument | HaneulArgument[])[] | undefined>) {
    return (
        <TransactionContent type={type}>
            {data && <>({flattenHaneulArguments(data)})</>}
        </TransactionContent>
    );
}

function MoveCall({ type, data }: TransactionProps<MoveCallHaneulTransaction>) {
    const {
        module,
        package: movePackage,
        function: func,
        arguments: args,
        type_arguments: typeArgs,
    } = data;
    return (
        <TransactionContent type={type}>
            (package: <ObjectLink objectId={movePackage} />, module:{' '}
            <ObjectLink
                objectId={`${movePackage}?module=${module}`}
                label={`'${module}'`}
            />
            , function: <span className="text-haneul-dark">{func}</span>
            {args && <>, arguments: [{flattenHaneulArguments(args!)}]</>}
            {typeArgs && <>, type_arguments: {typeArgs}</>})
        </TransactionContent>
    );
}

export function Transaction({
    type,
    data,
}: TransactionProps<
    (HaneulArgument | HaneulArgument[])[] | MoveCallHaneulTransaction | HaneulMovePackage
>) {
    if (type === 'MoveCall') {
        return (
            <ErrorBoundary>
                <MoveCall type={type} data={data as MoveCallHaneulTransaction} />
            </ErrorBoundary>
        );
    }

    return (
        <ErrorBoundary>
            <ArrayArgument
                type={type}
                data={
                    type !== 'Publish'
                        ? (data as (HaneulArgument | HaneulArgument[])[])
                        : undefined
                }
            />
        </ErrorBoundary>
    );
}
