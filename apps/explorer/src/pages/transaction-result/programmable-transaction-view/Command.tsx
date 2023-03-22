// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
    type MoveCallHaneulCommand,
    type HaneulArgument,
    type HaneulMovePackage,
} from '@haneullabs/haneul.js';
import { type ReactNode } from 'react';

import { flattenHaneulArguments } from './utils';

import { ObjectLink } from '~/ui/InternalLink';

export interface CommandProps<T> {
    type: string;
    data: T;
}

function CommandContent({
    type,
    children,
}: {
    type: string;
    children?: ReactNode;
}) {
    return (
        <>
            <div
                data-testid="programmable-transactions-command-label"
                className="text-heading6 font-semibold text-steel-darker"
            >
                {type}
            </div>
            {children && (
                <div
                    data-testid="programmable-transactions-command-content"
                    className="text-bodyMedium pt-2 font-medium text-steel-dark"
                >
                    {children}
                </div>
            )}
        </>
    );
}

function ArrayArgument({
    type,
    data,
}: CommandProps<(HaneulArgument | HaneulArgument[])[] | undefined>) {
    return (
        <CommandContent type={type}>
            {data && <>({flattenHaneulArguments(data)})</>}
        </CommandContent>
    );
}

function MoveCall({ type, data }: CommandProps<MoveCallHaneulCommand>) {
    const {
        module,
        package: movePackage,
        function: func,
        arguments: args,
        type_arguments: typeArgs,
    } = data;
    return (
        <CommandContent type={type}>
            (package: <ObjectLink objectId={movePackage} />, module:{' '}
            <ObjectLink
                objectId={`${movePackage}?module=${module}`}
                label={`'${module}'`}
            />
            , function: <span className="text-haneul-dark">{func}</span>
            {args && <>, arguments: [{flattenHaneulArguments(args!)}]</>}
            {typeArgs && <>, type_arguments: {typeArgs}</>})
        </CommandContent>
    );
}

export function Command({
    type,
    data,
}: CommandProps<
    (HaneulArgument | HaneulArgument[])[] | MoveCallHaneulCommand | HaneulMovePackage
>) {
    if (type === 'MoveCall') {
        return <MoveCall type={type} data={data as MoveCallHaneulCommand} />;
    }

    return (
        <ArrayArgument
            type={type}
            data={
                type !== 'Publish'
                    ? (data as (HaneulArgument | HaneulArgument[])[])
                    : undefined
            }
        />
    );
}
