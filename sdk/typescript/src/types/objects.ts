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

export type GetOwnedObjectRefsResponse = {
  objects: HaneulObjectRef[];
};

export type GetObjectInfoResponse = {
  status: ObjectStatus;
  details: HaneulObject | ObjectId | HaneulObjectRef;
};

export type ObjectDigest = string;
export type ObjectId = string;
export type SequenceNumber = number;

/* -------------------------------------------------------------------------- */
/*                              Helper functions                              */
/* -------------------------------------------------------------------------- */

/* -------------------------- GetObjectInfoResponse ------------------------- */

export function getObjectExistsResponse(
  resp: GetObjectInfoResponse
): HaneulObject | undefined {
  return resp.status !== 'Exists' ? undefined : (resp.details as HaneulObject);
}

export function getObjectDeletedResponse(
  resp: GetObjectInfoResponse
): HaneulObjectRef | undefined {
  return resp.status !== 'Deleted' ? undefined : (resp.details as HaneulObjectRef);
}

export function getObjectNotExistsResponse(
  resp: GetObjectInfoResponse
): ObjectId | undefined {
  return resp.status !== 'NotExists' ? undefined : (resp.details as ObjectId);
}

export function getObjectReference(
  resp: GetObjectInfoResponse
): HaneulObjectRef | undefined {
  return (
    getObjectExistsResponse(resp)?.reference || getObjectDeletedResponse(resp)
  );
}

/* ------------------------------ HaneulObjectRef ------------------------------ */

export function getObjectId(
  data: GetObjectInfoResponse | HaneulObjectRef
): ObjectId {
  if ('objectId' in data) {
    return data.objectId;
  }
  return (
    getObjectReference(data)?.objectId ?? getObjectNotExistsResponse(data)!
  );
}

export function getObjectVersion(
  data: GetObjectInfoResponse | HaneulObjectRef
): number | undefined {
  if ('version' in data) {
    return data.version;
  }
  return getObjectReference(data)?.version;
}

/* -------------------------------- HaneulObject ------------------------------- */

export function getObjectType(
  resp: GetObjectInfoResponse
): ObjectType | undefined {
  return getObjectExistsResponse(resp)?.data.dataType;
}

export function getObjectPreviousTransactionDigest(
  resp: GetObjectInfoResponse
): TransactionDigest | undefined {
  return getObjectExistsResponse(resp)?.previousTransaction;
}

export function getObjectOwner(
  resp: GetObjectInfoResponse
): ObjectOwner | undefined {
  return getObjectExistsResponse(resp)?.owner;
}

export function getMoveObjectType(
  resp: GetObjectInfoResponse
): string | undefined {
  return getMoveObject(resp)?.type;
}

export function getObjectFields(
  resp: GetObjectInfoResponse
): ObjectContentFields | undefined {
  return getMoveObject(resp)?.fields;
}

export function getMoveObject(
  resp: GetObjectInfoResponse
): HaneulMoveObject | undefined {
  const haneulObject = getObjectExistsResponse(resp);
  if (haneulObject?.data.dataType !== 'moveObject') {
    return undefined;
  }
  return haneulObject.data as HaneulMoveObject;
}

export function getMovePackageContent(
  data: GetObjectInfoResponse | HaneulMovePackage
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
