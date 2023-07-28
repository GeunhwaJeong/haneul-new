// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { CheckpointedObjectId } from './chain.js';
import type { HaneulMovePackage } from './move.js';

export type OwnedObjectRef = {
	owner: ObjectOwner;
	reference: HaneulObjectRef;
};

export type HaneulObjectRef = {
	/** Base64 string representing the object digest */
	digest: string;
	/** Hex code as string representing the object id */
	objectId: string;
	/** Object version */
	version: number | string;
};

export type ObjectOwner =
	| {
			AddressOwner: string;
	  }
	| {
			ObjectOwner: string;
	  }
	| {
			Shared: {
				initial_shared_version: number;
			};
	  }
	| 'Immutable';

export type HaneulObjectResponse = {
	data?: HaneulObjectData;
	error?: HaneulObjectResponseError;
};

export type HaneulObjectResponseError = {
	code: string;
	error?: string;
	object_id?: string;
	parent_object_id?: string;
	version?: number;
	digest?: string;
};

export type HaneulObjectData = {
	objectId: string;
	version: string;
	digest: string;
	/**
	 * Type of the object, default to be undefined unless HaneulObjectDataOptions.showType is set to true
	 */
	type?: string;
	/**
	 * Move object content or package content, default to be undefined unless HaneulObjectDataOptions.showContent is set to true
	 */
	content?: HaneulParsedData;
	/**
	 * Move object content or package content in BCS bytes, default to be undefined unless HaneulObjectDataOptions.showBcs is set to true
	 */
	bcs?: HaneulRawData;
	/**
	 * The owner of this object. Default to be undefined unless HaneulObjectDataOptions.showOwner is set to true
	 */
	owner?: ObjectOwner;
	/**
	 * The digest of the transaction that created or last mutated this object.
	 * Default to be undefined unless HaneulObjectDataOptions.showPreviousTransaction is set to true
	 */
	previousTransaction?: string;
	/**
	 * The amount of HANEUL we would rebate if this object gets deleted.
	 * This number is re-calculated each time the object is mutated based on
	 * the present storage gas price.
	 * Default to be undefined unless HaneulObjectDataOptions.showStorageRebate is set to true
	 */
	storageRebate?: string;
	/**
	 * Display metadata for this object, default to be undefined unless HaneulObjectDataOptions.showDisplay is set to true
	 * This can also be None if the struct type does not have Display defined
	 * See more details in https://forums.haneul.io/t/nft-object-display-proposal/4872
	 */
	display?: DisplayFieldsResponse;
};

export type HaneulParsedData =
	| (HaneulMoveObject & { dataType: 'moveObject' })
	| (HaneulMovePackage & { dataType: 'package' });

export type HaneulMoveObject = {
	/** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
	type: string;
	/** Fields and values stored inside the Move object */
	fields: ObjectContentFields;
	hasPublicTransfer: boolean;
};

export type HaneulRawData =
	| (HaneulRawMoveObject & { dataType: 'moveObject' })
	| (HaneulRawMovePackage & { dataType: 'package' });

export type HaneulRawMoveObject = {
	/** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
	type: string;
	hasPublicTransfer: boolean;
	version: number;
	bcsBytes: string;
};

export type HaneulRawMovePackage = {
	id: string;
	/** A mapping from module name to Move bytecode enocded in base64*/
	moduleMap: Record<string, string>;
};

export type DisplayFieldsResponse = {
	data: Record<string, string> | null;
	error: HaneulObjectResponseError | null;
};

export type ObjectContentFields = Record<string, any>;

export type PaginatedObjectsResponse = {
	data: HaneulObjectResponse[];
	// TODO: remove union after 0.30.0 is released
	nextCursor: string | CheckpointedObjectId | null;
	hasNextPage: boolean;
};

export type ObjectRead =
	| {
			details: HaneulObjectData;
			status: 'VersionFound';
	  }
	| {
			details: string;
			status: 'ObjectNotExists';
	  }
	| {
			details: HaneulObjectRef;
			status: 'ObjectDeleted';
	  }
	| {
			details: string | number;
			status: 'VersionNotFound';
	  }
	| {
			details: {
				asked_version: number;
				latest_version: number;
				object_id: string;
			};
			status: 'VersionTooHigh';
	  };

export type DynamicFieldName = {
	type: string;
	value?: any;
};

export type DynamicFieldInfo = {
	name: DynamicFieldName;
	bcsName: string;
	type: DynamicFieldType;
	objectType: string;
	objectId: string;
	version: number;
	digest: string;
};

export type DynamicFieldPage = {
	data: DynamicFieldInfo[];
	nextCursor: string | null;
	hasNextPage: boolean;
};

export type DynamicFieldType = 'DynamicField' | 'DynamicObject';
