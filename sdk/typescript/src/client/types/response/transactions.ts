// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulJsonValue } from '../common.js';
import type { GasCostSummary } from './chain.js';
import type { BalanceChange, HaneulEvent, HaneulObjectChange } from './events.js';
import type { HaneulMovePackage } from './move.js';
import type { OwnedObjectRef, HaneulObjectRef } from './objects.js';

export type TransactionEffects = {
	// Eventually this will become union(literal('v1'), literal('v2'), ...)
	messageVersion: 'v1';

	/** The status of the execution */
	status: ExecutionStatus;
	/** The epoch when this transaction was executed */
	executedEpoch: string;
	/** The version that every modified (mutated or deleted) object had before it was modified by this transaction. **/
	modifiedAtVersions?: TransactionEffectsModifiedAtVersions[];
	gasUsed: GasCostSummary;
	/** The object references of the shared objects used in this transaction. Empty if no shared objects were used. */
	sharedObjects?: HaneulObjectRef[];
	/** The transaction digest */
	transactionDigest: string;
	/** ObjectRef and owner of new objects created */
	created?: OwnedObjectRef[];
	/** ObjectRef and owner of mutated objects, including gas object */
	mutated?: OwnedObjectRef[];
	/**
	 * ObjectRef and owner of objects that are unwrapped in this transaction.
	 * Unwrapped objects are objects that were wrapped into other objects in the past,
	 * and just got extracted out.
	 */
	unwrapped?: OwnedObjectRef[];
	/** Object Refs of objects now deleted (the old refs) */
	deleted?: HaneulObjectRef[];
	/** Object Refs of objects now deleted (the old refs) */
	unwrappedThenDeleted?: HaneulObjectRef[];
	/** Object refs of objects now wrapped in other objects */
	wrapped?: HaneulObjectRef[];
	/**
	 * The updated gas object reference. Have a dedicated field for convenient access.
	 * It's also included in mutated.
	 */
	gasObject: OwnedObjectRef;
	/** The events emitted during execution. Note that only successful transactions emit events */
	eventsDigest?: string;
	/** The set of transaction digests this transaction depends on */
	dependencies?: string[];
};

export type ExecutionStatus = {
	status: ExecutionStatusType;
	error?: string;
};

export type ExecutionStatusType = 'success' | 'failure';

export type TransactionEffectsModifiedAtVersions = {
	objectId: string;
	sequenceNumber: string;
};

export type PaginatedTransactionResponse = {
	data: HaneulTransactionBlockResponse[];
	nextCursor: string | null;
	hasNextPage: boolean;
};

export type HaneulTransactionBlockResponse = {
	digest: string;
	transaction?: HaneulTransactionBlock;
	effects?: TransactionEffects;
	events?: HaneulEvent[];
	timestampMs?: string;
	checkpoint?: string;
	confirmedLocalExecution?: boolean;
	objectChanges?: HaneulObjectChange[];
	balanceChanges?: BalanceChange[];
	/* Errors that occurred in fetching/serializing the transaction. */
	errors?: string[];
};

export type HaneulTransactionBlock = {
	data: HaneulTransactionBlockData;
	txSignatures: string[];
};

export type HaneulTransactionBlockData = {
	// Eventually this will become union(literal('v1'), literal('v2'), ...)
	messageVersion: 'v1';
	transaction: HaneulTransactionBlockKind;
	sender: string;
	gasData: HaneulGasData;
};

export type HaneulTransactionBlockKind =
	| (HaneulChangeEpoch & { kind: 'ChangeEpoch' })
	| (HaneulConsensusCommitPrologue & {
			kind: 'ConsensusCommitPrologue';
	  })
	| (Genesis & { kind: 'Genesis' })
	| (ProgrammableTransaction & { kind: 'ProgrammableTransaction' });

export type HaneulChangeEpoch = {
	epoch: string;
	storage_charge: string;
	computation_charge: string;
	storage_rebate: string;
	epoch_start_timestamp_ms?: string;
};

export type Genesis = {
	objects: string[];
};

export type ProgrammableTransaction = {
	transactions: HaneulTransaction[];
	inputs: HaneulCallArg[];
};

export type HaneulCallArg =
	| {
			type: 'pure';
			valueType: string | null;
			value: HaneulJsonValue;
	  }
	| {
			type: 'object';
			objectType: 'immOrOwnedObject';
			objectId: string;
			version: string;
			digest: string;
	  }
	| {
			type: 'object';
			objectType: 'sharedObject';
			objectId: string;
			initialSharedVersion: string;
			mutable: boolean;
	  };

export type HaneulTransaction =
	| { MoveCall: MoveCallHaneulTransaction }
	| { TransferObjects: [HaneulArgument[], HaneulArgument] }
	| { SplitCoins: [HaneulArgument, HaneulArgument[]] }
	| { MergeCoins: [HaneulArgument, HaneulArgument[]] }
	| {
			Publish: // TODO: Remove this after 0.34 is released:
			[HaneulMovePackage, string[]] | string[];
	  }
	| {
			Upgrade: // TODO: Remove this after 0.34 is released:
			[HaneulMovePackage, string[], string, HaneulArgument] | [string[], string, HaneulArgument];
	  }
	| { MakeMoveVec: [string | null, HaneulArgument[]] };

export type HaneulArgument =
	| 'GasCoin'
	| { Input: number }
	| { Result: number }
	| { NestedResult: [number, number] };

export type MoveCallHaneulTransaction = {
	arguments?: HaneulArgument[];
	type_arguments?: string[];
	package: string;
	module: string;
	function: string;
};

export type HaneulGasData = {
	payment: HaneulObjectRef[];
	/** Gas Object's owner */
	owner: string;
	price: string;
	budget: string;
};

export type ExecutionResultType = {
	mutableReferenceOutputs?: MutableReferenceOutputType[];
	returnValues?: ReturnValueType[];
};

export type ReturnValueType = [number[], string];

export type MutableReferenceOutputType = [HaneulArgument, number[], string];

export type HaneulConsensusCommitPrologue = {
	epoch: string;
	round: string;
	commit_timestamp_ms: string;
};

export type DryRunTransactionBlockResponse = {
	effects: TransactionEffects;
	events: HaneulEvent[];
	objectChanges: HaneulObjectChange[];
	balanceChanges: BalanceChange[];
	// TODO: Remove optional when this is rolled out to all networks:
	input?: HaneulTransactionBlockData;
};

export type DevInspectResults = {
	effects: TransactionEffects;
	events: HaneulEvent[];
	results?: ExecutionResultType[];
	error?: string;
};
