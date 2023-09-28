// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Infer } from 'superstruct';
import {
	any,
	array,
	assign,
	bigint,
	boolean,
	is,
	literal,
	nullable,
	number,
	object,
	optional,
	record,
	string,
	tuple,
	union,
	unknown,
} from 'superstruct';

import { ObjectOwner } from './common.js';
import type { OwnedObjectRef } from './transactions.js';

export const ObjectType = union([string(), literal('package')]);
export type ObjectType = Infer<typeof ObjectType>;

export const HaneulObjectRef = object({
	/** Base64 string representing the object digest */
	digest: string(),
	/** Hex code as string representing the object id */
	objectId: string(),
	/** Object version */
	version: union([number(), string(), bigint()]),
});
export type HaneulObjectRef = Infer<typeof HaneulObjectRef>;

export const HaneulGasData = object({
	payment: array(HaneulObjectRef),
	/** Gas Object's owner */
	owner: string(),
	price: string(),
	budget: string(),
});
export type HaneulGasData = Infer<typeof HaneulGasData>;

export const HaneulObjectInfo = assign(
	HaneulObjectRef,
	object({
		type: string(),
		owner: ObjectOwner,
		previousTransaction: string(),
	}),
);
export type HaneulObjectInfo = Infer<typeof HaneulObjectInfo>;

export const ObjectContentFields = record(string(), any());
export type ObjectContentFields = Infer<typeof ObjectContentFields>;

export const MovePackageContent = record(string(), unknown());
export type MovePackageContent = Infer<typeof MovePackageContent>;

export const HaneulMoveObject = object({
	/** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
	type: string(),
	/** Fields and values stored inside the Move object */
	fields: ObjectContentFields,
	hasPublicTransfer: boolean(),
});
export type HaneulMoveObject = Infer<typeof HaneulMoveObject>;

export const HaneulMovePackage = object({
	/** A mapping from module name to disassembled Move bytecode */
	disassembled: MovePackageContent,
});
export type HaneulMovePackage = Infer<typeof HaneulMovePackage>;

export const HaneulParsedData = union([
	assign(HaneulMoveObject, object({ dataType: literal('moveObject') })),
	assign(HaneulMovePackage, object({ dataType: literal('package') })),
]);
export type HaneulParsedData = Infer<typeof HaneulParsedData>;

export const HaneulRawMoveObject = object({
	/** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
	type: string(),
	hasPublicTransfer: boolean(),
	version: string(),
	bcsBytes: string(),
});
export type HaneulRawMoveObject = Infer<typeof HaneulRawMoveObject>;

export const HaneulRawMovePackage = object({
	id: string(),
	/** A mapping from module name to Move bytecode enocded in base64*/
	moduleMap: record(string(), string()),
});
export type HaneulRawMovePackage = Infer<typeof HaneulRawMovePackage>;

// TODO(chris): consolidate HaneulRawParsedData and HaneulRawObject using generics
export const HaneulRawData = union([
	assign(HaneulRawMoveObject, object({ dataType: literal('moveObject') })),
	assign(HaneulRawMovePackage, object({ dataType: literal('package') })),
]);
export type HaneulRawData = Infer<typeof HaneulRawData>;

export const HANEUL_DECIMALS = 9;

export const GEUNHWA_PER_HANEUL = BigInt(1000000000);

/** @deprecated Use `string` instead. */
export const ObjectDigest = string();
/** @deprecated Use `string` instead. */
export type ObjectDigest = Infer<typeof ObjectDigest>;

export const HaneulObjectResponseError = object({
	code: string(),
	error: optional(string()),
	object_id: optional(string()),
	parent_object_id: optional(string()),
	version: optional(string()),
	digest: optional(string()),
});
export type HaneulObjectResponseError = Infer<typeof HaneulObjectResponseError>;
export const DisplayFieldsResponse = object({
	data: nullable(optional(record(string(), string()))),
	error: nullable(optional(HaneulObjectResponseError)),
});
export type DisplayFieldsResponse = Infer<typeof DisplayFieldsResponse>;
// TODO: remove after all envs support the new DisplayFieldsResponse;
export const DisplayFieldsBackwardCompatibleResponse = union([
	DisplayFieldsResponse,
	optional(record(string(), string())),
]);
export type DisplayFieldsBackwardCompatibleResponse = Infer<
	typeof DisplayFieldsBackwardCompatibleResponse
>;

export const HaneulObjectData = object({
	objectId: string(),
	version: string(),
	digest: string(),
	/**
	 * Type of the object, default to be undefined unless HaneulObjectDataOptions.showType is set to true
	 */
	type: nullable(optional(string())),
	/**
	 * Move object content or package content, default to be undefined unless HaneulObjectDataOptions.showContent is set to true
	 */
	content: nullable(optional(HaneulParsedData)),
	/**
	 * Move object content or package content in BCS bytes, default to be undefined unless HaneulObjectDataOptions.showBcs is set to true
	 */
	bcs: nullable(optional(HaneulRawData)),
	/**
	 * The owner of this object. Default to be undefined unless HaneulObjectDataOptions.showOwner is set to true
	 */
	owner: nullable(optional(ObjectOwner)),
	/**
	 * The digest of the transaction that created or last mutated this object.
	 * Default to be undefined unless HaneulObjectDataOptions.showPreviousTransaction is set to true
	 */
	previousTransaction: nullable(optional(string())),
	/**
	 * The amount of HANEUL we would rebate if this object gets deleted.
	 * This number is re-calculated each time the object is mutated based on
	 * the present storage gas price.
	 * Default to be undefined unless HaneulObjectDataOptions.showStorageRebate is set to true
	 */
	storageRebate: nullable(optional(string())),
	/**
	 * Display metadata for this object, default to be undefined unless HaneulObjectDataOptions.showDisplay is set to true
	 * This can also be None if the struct type does not have Display defined
	 * See more details in https://forums.haneul.io/t/nft-object-display-proposal/4872
	 */
	display: nullable(optional(DisplayFieldsBackwardCompatibleResponse)),
});
export type HaneulObjectData = Infer<typeof HaneulObjectData>;

/**
 * Config for fetching object data
 */
export const HaneulObjectDataOptions = object({
	/* Whether to fetch the object type, default to be true */
	showType: nullable(optional(boolean())),
	/* Whether to fetch the object content, default to be false */
	showContent: nullable(optional(boolean())),
	/* Whether to fetch the object content in BCS bytes, default to be false */
	showBcs: nullable(optional(boolean())),
	/* Whether to fetch the object owner, default to be false */
	showOwner: nullable(optional(boolean())),
	/* Whether to fetch the previous transaction digest, default to be false */
	showPreviousTransaction: nullable(optional(boolean())),
	/* Whether to fetch the storage rebate, default to be false */
	showStorageRebate: nullable(optional(boolean())),
	/* Whether to fetch the display metadata, default to be false */
	showDisplay: nullable(optional(boolean())),
});
export type HaneulObjectDataOptions = Infer<typeof HaneulObjectDataOptions>;

export const ObjectStatus = union([literal('Exists'), literal('notExists'), literal('Deleted')]);
export type ObjectStatus = Infer<typeof ObjectStatus>;

export const GetOwnedObjectsResponse = array(HaneulObjectInfo);
export type GetOwnedObjectsResponse = Infer<typeof GetOwnedObjectsResponse>;

export const HaneulObjectResponse = object({
	data: nullable(optional(HaneulObjectData)),
	error: nullable(optional(HaneulObjectResponseError)),
});
export type HaneulObjectResponse = Infer<typeof HaneulObjectResponse>;

export type Order = 'ascending' | 'descending';

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* -------------------------- HaneulObjectResponse ------------------------- */

export function getHaneulObjectData(resp: HaneulObjectResponse): HaneulObjectData | null | undefined {
	return resp.data;
}

export function getObjectDeletedResponse(resp: HaneulObjectResponse): HaneulObjectRef | undefined {
	if (
		resp.error &&
		'object_id' in resp.error &&
		'version' in resp.error &&
		'digest' in resp.error
	) {
		const error = resp.error as HaneulObjectResponseError;
		return {
			objectId: error.object_id,
			version: error.version,
			digest: error.digest,
		} as HaneulObjectRef;
	}

	return undefined;
}

export function getObjectNotExistsResponse(resp: HaneulObjectResponse): string | undefined {
	if (
		resp.error &&
		'object_id' in resp.error &&
		!('version' in resp.error) &&
		!('digest' in resp.error)
	) {
		return (resp.error as HaneulObjectResponseError).object_id as string;
	}

	return undefined;
}

export function getObjectReference(
	resp: HaneulObjectResponse | OwnedObjectRef,
): HaneulObjectRef | undefined {
	if ('reference' in resp) {
		return resp.reference;
	}
	const exists = getHaneulObjectData(resp);
	if (exists) {
		return {
			objectId: exists.objectId,
			version: exists.version,
			digest: exists.digest,
		};
	}
	return getObjectDeletedResponse(resp);
}

/* ------------------------------ HaneulObjectRef ------------------------------ */

export function getObjectId(data: HaneulObjectResponse | HaneulObjectRef | OwnedObjectRef): string {
	if ('objectId' in data) {
		return data.objectId;
	}
	return (
		getObjectReference(data)?.objectId ?? getObjectNotExistsResponse(data as HaneulObjectResponse)!
	);
}

export function getObjectVersion(
	data: HaneulObjectResponse | HaneulObjectRef | HaneulObjectData,
): string | number | bigint | undefined {
	if ('version' in data) {
		return data.version;
	}
	return getObjectReference(data)?.version;
}

/* -------------------------------- HaneulObject ------------------------------- */

export function isHaneulObjectResponse(
	resp: HaneulObjectResponse | HaneulObjectData,
): resp is HaneulObjectResponse {
	return (resp as HaneulObjectResponse).data !== undefined;
}

/**
 * Deriving the object type from the object response
 * @returns 'package' if the object is a package, move object type(e.g., 0x2::coin::Coin<0x2::haneul::HANEUL>)
 * if the object is a move object
 */
export function getObjectType(
	resp: HaneulObjectResponse | HaneulObjectData,
): ObjectType | null | undefined {
	const data = isHaneulObjectResponse(resp) ? resp.data : resp;

	if (!data?.type && 'data' in resp) {
		if (data?.content?.dataType === 'package') {
			return 'package';
		}
		return getMoveObjectType(resp);
	}
	return data?.type;
}

export function getObjectPreviousTransactionDigest(
	resp: HaneulObjectResponse,
): string | null | undefined {
	return getHaneulObjectData(resp)?.previousTransaction;
}

export function getObjectOwner(
	resp: HaneulObjectResponse | ObjectOwner,
): ObjectOwner | null | undefined {
	if (is(resp, ObjectOwner)) {
		return resp;
	}
	return getHaneulObjectData(resp)?.owner;
}

export function getObjectDisplay(resp: HaneulObjectResponse): DisplayFieldsResponse {
	const display = getHaneulObjectData(resp)?.display;
	if (!display) {
		return { data: null, error: null };
	}
	if (is(display, DisplayFieldsResponse)) {
		return display;
	}
	return {
		data: display,
		error: null,
	};
}

export function getSharedObjectInitialVersion(
	resp: HaneulObjectResponse | ObjectOwner,
): string | null | undefined {
	const owner = getObjectOwner(resp);
	if (owner && typeof owner === 'object' && 'Shared' in owner) {
		return owner.Shared.initial_shared_version;
	} else {
		return undefined;
	}
}

export function isSharedObject(resp: HaneulObjectResponse | ObjectOwner): boolean {
	const owner = getObjectOwner(resp);
	return !!owner && typeof owner === 'object' && 'Shared' in owner;
}

export function isImmutableObject(resp: HaneulObjectResponse | ObjectOwner): boolean {
	const owner = getObjectOwner(resp);
	return owner === 'Immutable';
}

export function getMoveObjectType(resp: HaneulObjectResponse): string | undefined {
	return getMoveObject(resp)?.type;
}

export function getObjectFields(
	resp: HaneulObjectResponse | HaneulMoveObject | HaneulObjectData,
): ObjectContentFields | undefined {
	if ('fields' in resp) {
		return resp.fields;
	}
	return getMoveObject(resp)?.fields;
}

export interface HaneulObjectDataWithContent extends HaneulObjectData {
	content: HaneulParsedData;
}

function isHaneulObjectDataWithContent(data: HaneulObjectData): data is HaneulObjectDataWithContent {
	return data.content !== undefined;
}

export function getMoveObject(data: HaneulObjectResponse | HaneulObjectData): HaneulMoveObject | undefined {
	const haneulObject = 'data' in data ? getHaneulObjectData(data) : (data as HaneulObjectData);

	if (
		!haneulObject ||
		!isHaneulObjectDataWithContent(haneulObject) ||
		haneulObject.content.dataType !== 'moveObject'
	) {
		return undefined;
	}

	return haneulObject.content as HaneulMoveObject;
}

export function hasPublicTransfer(data: HaneulObjectResponse | HaneulObjectData): boolean {
	return getMoveObject(data)?.hasPublicTransfer ?? false;
}

export function getMovePackageContent(
	data: HaneulObjectResponse | HaneulMovePackage,
): MovePackageContent | undefined {
	if ('disassembled' in data) {
		return data.disassembled;
	}
	const haneulObject = getHaneulObjectData(data);
	if (haneulObject?.content?.dataType !== 'package') {
		return undefined;
	}
	return (haneulObject.content as HaneulMovePackage).disassembled;
}

export const CheckpointedObjectId = object({
	objectId: string(),
	atCheckpoint: optional(number()),
});
export type CheckpointedObjectId = Infer<typeof CheckpointedObjectId>;

export const PaginatedObjectsResponse = object({
	data: array(HaneulObjectResponse),
	nextCursor: optional(nullable(string())),
	hasNextPage: boolean(),
});
export type PaginatedObjectsResponse = Infer<typeof PaginatedObjectsResponse>;

// mirrors haneul_json_rpc_types:: HaneulObjectDataFilter
export type HaneulObjectDataFilter =
	| { MatchAll: HaneulObjectDataFilter[] }
	| { MatchAny: HaneulObjectDataFilter[] }
	| { MatchNone: HaneulObjectDataFilter[] }
	| { Package: string }
	| { MoveModule: { package: string; module: string } }
	| { StructType: string }
	| { AddressOwner: string }
	| { ObjectOwner: string }
	| { ObjectId: string }
	| { ObjectIds: string[] }
	| { Version: string };

export type HaneulObjectResponseQuery = {
	filter?: HaneulObjectDataFilter;
	options?: HaneulObjectDataOptions;
};

export const ObjectRead = union([
	object({
		details: HaneulObjectData,
		status: literal('VersionFound'),
	}),
	object({
		details: string(),
		status: literal('ObjectNotExists'),
	}),
	object({
		details: HaneulObjectRef,
		status: literal('ObjectDeleted'),
	}),
	object({
		details: tuple([string(), number()]),
		status: literal('VersionNotFound'),
	}),
	object({
		details: object({
			asked_version: number(),
			latest_version: number(),
			object_id: string(),
		}),
		status: literal('VersionTooHigh'),
	}),
]);
export type ObjectRead = Infer<typeof ObjectRead>;
