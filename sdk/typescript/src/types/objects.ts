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
  /** Move type (e.g., "0x2::coin::Coin<0x2::haneul::HANEUL>") */
  type: string;
  /** Fields and values stored inside the Move object */
  fields: ObjectContentFields;
  has_public_transfer?: boolean;
};

export type HaneulMovePackage = {
  /** A mapping from module name to disassembled Move bytecode */
  disassembled: MovePackageContent;
};

export type HaneulMoveFunctionArgTypesResponse = HaneulMoveFunctionArgType[];

export type HaneulMoveFunctionArgType = string | { Object: string };

export type HaneulMoveFunctionArgTypes = HaneulMoveFunctionArgType[];

export type HaneulMoveNormalizedModules = Record<string, HaneulMoveNormalizedModule>;

export type HaneulMoveNormalizedModule = {
  file_format_version: number;
  address: string;
  name: string;
  friends: HaneulMoveModuleId[];
  structs: Record<string, HaneulMoveNormalizedStruct>;
  exposed_functions: Record<string, HaneulMoveNormalizedFunction>;
};

export type HaneulMoveModuleId = {
  address: string;
  name: string;
};

export type HaneulMoveNormalizedStruct = {
  abilities: HaneulMoveAbilitySet;
  type_parameters: HaneulMoveStructTypeParameter[];
  fields: HaneulMoveNormalizedField[];
};

export type HaneulMoveStructTypeParameter = {
  constraints: HaneulMoveAbilitySet;
  is_phantom: boolean;
};

export type HaneulMoveNormalizedField = {
  name: string;
  type_: HaneulMoveNormalizedType;
};

export type HaneulMoveNormalizedFunction = {
  visibility: HaneulMoveVisibility;
  is_entry: boolean;
  type_parameters: HaneulMoveAbilitySet[];
  parameters: HaneulMoveNormalizedType[];
  return_: HaneulMoveNormalizedType[];
};

export type HaneulMoveVisibility = 'Private' | 'Public' | 'Friend';

export type HaneulMoveTypeParameterIndex = number;

export type HaneulMoveAbilitySet = {
  abilities: string[];
};

export type HaneulMoveNormalizedType =
  | string
  | HaneulMoveNormalizedTypeParameterType
  | { Reference: HaneulMoveNormalizedStructType }
  | { MutableReference: HaneulMoveNormalizedStructType }
  | { Vector: HaneulMoveNormalizedType }
  | HaneulMoveNormalizedStructType;

export type HaneulMoveNormalizedTypeParameterType = {
  TypeParameter: HaneulMoveTypeParameterIndex;
};

export type HaneulMoveNormalizedStructType = {
  Struct: {
    address: string;
    module: string;
    name: string;
    type_arguments: HaneulMoveNormalizedTypeParameterType[];
  };
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

export function isSharedObject(resp: GetObjectDataResponse): boolean {
  const owner = getObjectOwner(resp);
  return owner === 'Shared';
}

export function isImmutableObject(resp: GetObjectDataResponse): boolean {
  const owner = getObjectOwner(resp);
  return owner === 'Immutable';
}

export function getMoveObjectType(
  resp: GetObjectDataResponse
): string | undefined {
  return getMoveObject(resp)?.type;
}

export function getObjectFields(
  resp: GetObjectDataResponse | HaneulMoveObject
): ObjectContentFields | undefined {
  if ('fields' in resp) {
    return resp.fields;
  }
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

export function hasPublicTransfer(
  data: GetObjectDataResponse | HaneulObject
): boolean {
  return getMoveObject(data)?.has_public_transfer ?? false;
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

export function extractMutableReference(
  normalizedType: HaneulMoveNormalizedType
): HaneulMoveNormalizedStructType | undefined {
  return typeof normalizedType === 'object' &&
    'MutableReference' in normalizedType
    ? normalizedType.MutableReference
    : undefined;
}

export function extractReference(
  normalizedType: HaneulMoveNormalizedType
): HaneulMoveNormalizedStructType | undefined {
  return typeof normalizedType === 'object' && 'Reference' in normalizedType
    ? normalizedType.Reference
    : undefined;
}

export function extractStructTag(
  normalizedType: HaneulMoveNormalizedType
): HaneulMoveNormalizedStructType | undefined {
  if (typeof normalizedType === 'object' && 'Struct' in normalizedType) {
    return normalizedType;
  }

  return (
    (extractReference(normalizedType) ||
      extractMutableReference(normalizedType)) ??
    undefined
  );
}
