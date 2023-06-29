// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { type MoveCallHaneulTransaction, type HaneulArgument, type HaneulMovePackage } from '@haneullabs/haneul.js';
import { type ReactNode } from 'react';

import { flattenHaneulArguments } from './utils';
import { ErrorBoundary } from '~/components/error-boundary/ErrorBoundary';
import { ObjectLink } from '~/ui/InternalLink';
import { Text } from '~/ui/Text';

export interface TransactionProps<T> {
	type: string;
	data: T;
}

function TransactionContent({ children }: { children?: ReactNode }) {
	return (
		<Text variant="pBody/normal" color="steel-dark">
			{children}
		</Text>
	);
}

function ArrayArgument({ data }: TransactionProps<(HaneulArgument | HaneulArgument[])[] | undefined>) {
	return (
		<TransactionContent>
			{data && (
				<span className="break-all">
					<Text variant="pBody/medium">({flattenHaneulArguments(data)})</Text>
				</span>
			)}
		</TransactionContent>
	);
}

function MoveCall({ data }: TransactionProps<MoveCallHaneulTransaction>) {
	const {
		module,
		package: movePackage,
		function: func,
		arguments: args,
		type_arguments: typeArgs,
	} = data;

	return (
		<TransactionContent>
			<Text variant="pBody/medium">
				(package: <ObjectLink objectId={movePackage} />, module:{' '}
				<ObjectLink objectId={`${movePackage}?module=${module}`} label={`'${module}'`} />, function:{' '}
				<span className="break-all text-hero-dark">{func}</span>
				{args && <span className="break-all">, arguments: [{flattenHaneulArguments(args!)}]</span>}
				{typeArgs && <span className="break-all">, type_arguments: [{typeArgs.join(', ')}]</span>}
			</Text>
		</TransactionContent>
	);
}

export function Transaction({
	type,
	data,
}: TransactionProps<(HaneulArgument | HaneulArgument[])[] | MoveCallHaneulTransaction | HaneulMovePackage>) {
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
				data={type !== 'Publish' ? (data as (HaneulArgument | HaneulArgument[])[]) : undefined}
			/>
		</ErrorBoundary>
	);
}
