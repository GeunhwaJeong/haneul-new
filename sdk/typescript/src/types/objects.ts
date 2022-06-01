// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { ObjectOwner } from './common';
import { TransactionDigest } from './common';

export type HaneulObjectRef = {
  /** Base64 string representing the object digest */
  digest: TransactionDigest;
  /** Hex code as string representing the object id */
  objectId: string;
  /** Object version */
  version: number;
};

export type HaneulObjectInfo = HaneulObjectRef & {
  type: string;
  owner: ObjectOwner;
  previousTransaction: TransactionDigest;
};

export type ObjectContentFields = Record<string, any>;

export type MovePackageContent = Record<string, string>;

export type HaneulData = { dataType: ObjectType } & (
  | HaneulMoveObject
  | HaneulMovePackage
);

export type HaneulMoveObject = {
  /** Move type (e.g., "0x2::Coin::Coin<0x2::HANEUL::HANEUL>") */
  type: string;
  /** Fields and values stored inside the Move object */
  fields: ObjectContentFields;
};

export type HaneulMovePackage = {
  /** A mapping from module name to disassembled Move bytecode */
  disassembled: MovePackageContent;
};

export type HaneulObject = {
  /** The meat of the object */
  data: HaneulData;
  /** The owner of the object */
  owner: ObjectOwner;
  /** The digest of the transaction that created or last mutated this object */
  previousTransaction: TransactionDigest;
  /**
   * The amount of HANEUL we would rebate if this object gets deleted.
   * This number is re-calculated each time the object is mutated based on
   * the present storage gas price.
   */
  storageRebate: number;
  reference: HaneulObjectRef;
};

export type ObjectStatus = 'Exists' | 'NotExists' | 'Deleted';
export type ObjectType = 'moveObject' | 'package';

export type GetOwnedObjectsResponse = HaneulObjectInfo[];

export type GetObjectDataResponse = {
  status: ObjectStatus;
  details: HaneulObject | ObjectId | HaneulObjectRef;
};

export type ObjectDigest = string;
export type ObjectId = string;
export type SequenceNumber = number;

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* -------------------------- GetObjectDataResponse ------------------------- */

export function getObjectExistsResponse(
  resp: GetObjectDataResponse
): HaneulObject | undefined {
  return resp.status !== 'Exists' ? undefined : (resp.details as HaneulObject);
}

export function getObjectDeletedResponse(
  resp: GetObjectDataResponse
): HaneulObjectRef | undefined {
  return resp.status !== 'Deleted' ? undefined : (resp.details as HaneulObjectRef);
}

export function getObjectNotExistsResponse(
  resp: GetObjectDataResponse
): ObjectId | undefined {
  return resp.status !== 'NotExists' ? undefined : (resp.details as ObjectId);
}

export function getObjectReference(
  resp: GetObjectDataResponse
): HaneulObjectRef | undefined {
  return (
    getObjectExistsResponse(resp)?.reference || getObjectDeletedResponse(resp)
  );
}

/* ------------------------------ HaneulObjectRef ------------------------------ */

export function getObjectId(
  data: GetObjectDataResponse | HaneulObjectRef
): ObjectId {
  if ('objectId' in data) {
    return data.objectId;
  }
  return (
    getObjectReference(data)?.objectId ?? getObjectNotExistsResponse(data)!
  );
}

export function getObjectVersion(
  data: GetObjectDataResponse | HaneulObjectRef
): number | undefined {
  if ('version' in data) {
    return data.version;
  }
  return getObjectReference(data)?.version;
}

/* -------------------------------- HaneulObject ------------------------------- */

export function getObjectType(
  resp: GetObjectDataResponse
): ObjectType | undefined {
  return getObjectExistsResponse(resp)?.data.dataType;
}

export function getObjectPreviousTransactionDigest(
  resp: GetObjectDataResponse
): TransactionDigest | undefined {
  return getObjectExistsResponse(resp)?.previousTransaction;
}

export function getObjectOwner(
  resp: GetObjectDataResponse
): ObjectOwner | undefined {
  return getObjectExistsResponse(resp)?.owner;
}

export function getMoveObjectType(
  resp: GetObjectDataResponse
): string | undefined {
  return getMoveObject(resp)?.type;
}

export function getObjectFields(
  resp: GetObjectDataResponse
): ObjectContentFields | undefined {
  return getMoveObject(resp)?.fields;
}

export function getMoveObject(
  data: GetObjectDataResponse | HaneulObject
): HaneulMoveObject | undefined {
  const haneulObject = 'data' in data ? data : getObjectExistsResponse(data);
  if (haneulObject?.data.dataType !== 'moveObject') {
    return undefined;
  }
  return haneulObject.data as HaneulMoveObject;
}

export function getMovePackageContent(
  data: GetObjectDataResponse | HaneulMovePackage
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
