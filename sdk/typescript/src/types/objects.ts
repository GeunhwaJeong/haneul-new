// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  any,
  array,
  assign,
  boolean,
  Infer,
  literal,
  number,
  object,
  optional,
  record,
  string,
  union,
} from 'superstruct';
import { ObjectId, ObjectOwner, TransactionDigest } from './common';

export const ObjectType = union([literal('moveObject'), literal('package')]);
export type ObjectType = Infer<typeof ObjectType>;

export const HaneulObjectRef = object({
  /** Base64 string representing the object digest */
  digest: TransactionDigest,
  /** Hex code as string representing the object id */
  objectId: string(),
  /** Object version */
  version: number(),
});
export type HaneulObjectRef = Infer<typeof HaneulObjectRef>;

export const HaneulObjectInfo = assign(
  HaneulObjectRef,
  object({
    type: string(),
    owner: ObjectOwner,
    previousTransaction: TransactionDigest,
  }),
);
export type HaneulObjectInfo = Infer<typeof HaneulObjectInfo>;

export const ObjectContentFields = record(string(), any());
export type ObjectContentFields = Infer<typeof ObjectContentFields>;

export const MovePackageContent = record(string(), string());
export type MovePackageContent = Infer<typeof MovePackageContent>;

export const HaneulMoveObject = object({
  /** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
  type: string(),
  /** Fields and values stored inside the Move object */
  fields: ObjectContentFields,
  has_public_transfer: optional(boolean()),
});
export type HaneulMoveObject = Infer<typeof HaneulMoveObject>;

export const HaneulMovePackage = object({
  /** A mapping from module name to disassembled Move bytecode */
  disassembled: MovePackageContent,
});
export type HaneulMovePackage = Infer<typeof HaneulMovePackage>;

export const HaneulData = union([
  assign(HaneulMoveObject, object({ dataType: literal('moveObject') })),
  assign(HaneulMovePackage, object({ dataType: literal('package') })),
]);
export type HaneulData = Infer<typeof HaneulData>;

export const GEUNHWA_PER_HANEUL = BigInt(1000000000);

export const HaneulObject = object({
  /** The meat of the object */
  data: HaneulData,
  /** The owner of the object */
  owner: ObjectOwner,
  /** The digest of the transaction that created or last mutated this object */
  previousTransaction: TransactionDigest,
  /**
   * The amount of HANEUL we would rebate if this object gets deleted.
   * This number is re-calculated each time the object is mutated based on
   * the present storage gas price.
   */
  storageRebate: number(),
  reference: HaneulObjectRef,
});
export type HaneulObject = Infer<typeof HaneulObject>;

export const ObjectStatus = union([
  literal('Exists'),
  literal('NotExists'),
  literal('Deleted'),
]);
export type ObjectStatus = Infer<typeof ObjectStatus>;

export const GetOwnedObjectsResponse = array(HaneulObjectInfo);
export type GetOwnedObjectsResponse = Infer<typeof GetOwnedObjectsResponse>;

export const GetObjectDataResponse = object({
  status: ObjectStatus,
  details: union([HaneulObject, ObjectId, HaneulObjectRef]),
});
export type GetObjectDataResponse = Infer<typeof GetObjectDataResponse>;

export type ObjectDigest = string;
export type Order = 'ascending' | 'descending';

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* -------------------------- GetObjectDataResponse ------------------------- */

export function getObjectExistsResponse(
  resp: GetObjectDataResponse,
): HaneulObject | undefined {
  return resp.status !== 'Exists' ? undefined : (resp.details as HaneulObject);
}

export function getObjectDeletedResponse(
  resp: GetObjectDataResponse,
): HaneulObjectRef | undefined {
  return resp.status !== 'Deleted' ? undefined : (resp.details as HaneulObjectRef);
}

export function getObjectNotExistsResponse(
  resp: GetObjectDataResponse,
): ObjectId | undefined {
  return resp.status !== 'NotExists' ? undefined : (resp.details as ObjectId);
}

export function getObjectReference(
  resp: GetObjectDataResponse,
): HaneulObjectRef | undefined {
  return (
    getObjectExistsResponse(resp)?.reference || getObjectDeletedResponse(resp)
  );
}

/* ------------------------------ HaneulObjectRef ------------------------------ */

export function getObjectId(
  data: GetObjectDataResponse | HaneulObjectRef,
): ObjectId {
  if ('objectId' in data) {
    return data.objectId;
  }
  return (
    getObjectReference(data)?.objectId ?? getObjectNotExistsResponse(data)!
  );
}

export function getObjectVersion(
  data: GetObjectDataResponse | HaneulObjectRef,
): number | undefined {
  if ('version' in data) {
    return data.version;
  }
  return getObjectReference(data)?.version;
}

/* -------------------------------- HaneulObject ------------------------------- */

export function getObjectType(
  resp: GetObjectDataResponse,
): ObjectType | undefined {
  return getObjectExistsResponse(resp)?.data.dataType;
}

export function getObjectPreviousTransactionDigest(
  resp: GetObjectDataResponse,
): TransactionDigest | undefined {
  return getObjectExistsResponse(resp)?.previousTransaction;
}

export function getObjectOwner(
  resp: GetObjectDataResponse,
): ObjectOwner | undefined {
  return getObjectExistsResponse(resp)?.owner;
}

export function getSharedObjectInitialVersion(
  resp: GetObjectDataResponse,
): number | undefined {
  const owner = getObjectOwner(resp);
  if (typeof owner === 'object' && 'Shared' in owner) {
    return owner.Shared.initial_shared_version;
  } else {
    return undefined;
  }
}

export function isSharedObject(resp: GetObjectDataResponse): boolean {
  const owner = getObjectOwner(resp);
  return typeof owner === 'object' && 'Shared' in owner;
}

export function isImmutableObject(resp: GetObjectDataResponse): boolean {
  const owner = getObjectOwner(resp);
  return owner === 'Immutable';
}

export function getMoveObjectType(
  resp: GetObjectDataResponse,
): string | undefined {
  return getMoveObject(resp)?.type;
}

export function getObjectFields(
  resp: GetObjectDataResponse | HaneulMoveObject,
): ObjectContentFields | undefined {
  if ('fields' in resp) {
    return resp.fields;
  }
  return getMoveObject(resp)?.fields;
}

export function getMoveObject(
  data: GetObjectDataResponse | HaneulObject,
): HaneulMoveObject | undefined {
  const haneulObject = 'data' in data ? data : getObjectExistsResponse(data);
  if (haneulObject?.data.dataType !== 'moveObject') {
    return undefined;
  }
  return haneulObject.data as HaneulMoveObject;
}

export function hasPublicTransfer(
  data: GetObjectDataResponse | HaneulObject,
): boolean {
  return getMoveObject(data)?.has_public_transfer ?? false;
}

export function getMovePackageContent(
  data: GetObjectDataResponse | HaneulMovePackage,
): MovePackageContent | undefined {
  if ('disassembled' in data) {
    return data.disassembled;
  }
  const haneulObject = getObjectExistsResponse(data);
  if (haneulObject?.data.dataType !== 'package') {
    return undefined;
  }
  return (haneulObject.data as HaneulMovePackage).disassembled;
}
