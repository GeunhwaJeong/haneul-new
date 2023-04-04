// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  array,
  boolean,
  Infer,
  integer,
  object,
  string,
  union,
} from 'superstruct';
import {
  normalizeHaneulAddress,
  ObjectId,
  SharedObjectRef,
  HaneulObjectRef,
} from '../types';
import { builder } from './bcs';

const ObjectArg = union([
  object({ ImmOrOwned: HaneulObjectRef }),
  object({
    Shared: object({
      objectId: string(),
      initialSharedVersion: union([integer(), string()]),
      mutable: boolean(),
    }),
  }),
]);

export const PureCallArg = object({ Pure: array(integer()) });
export const ObjectCallArg = object({ Object: ObjectArg });
export type PureCallArg = Infer<typeof PureCallArg>;
export type ObjectCallArg = Infer<typeof ObjectCallArg>;

export const BuilderCallArg = union([PureCallArg, ObjectCallArg]);
export type BuilderCallArg = Infer<typeof BuilderCallArg>;

export const Inputs = {
  Pure(data: unknown, type?: string): PureCallArg {
    return {
      Pure: Array.from(
        data instanceof Uint8Array ? data : builder.ser(type!, data).toBytes(),
      ),
    };
  },
  ObjectRef({ objectId, digest, version }: HaneulObjectRef): ObjectCallArg {
    return {
      Object: {
        ImmOrOwned: {
          digest,
          version,
          objectId: normalizeHaneulAddress(objectId),
        },
      },
    };
  },
  SharedObjectRef({
    objectId,
    mutable,
    initialSharedVersion,
  }: SharedObjectRef): ObjectCallArg {
    return {
      Object: {
        Shared: {
          mutable,
          initialSharedVersion,
          objectId: normalizeHaneulAddress(objectId),
        },
      },
    };
  },
};

export function getIdFromCallArg(arg: ObjectId | ObjectCallArg) {
  if (typeof arg === 'string') {
    return normalizeHaneulAddress(arg);
  }
  if ('ImmOrOwned' in arg.Object) {
    return normalizeHaneulAddress(arg.Object.ImmOrOwned.objectId);
  }
  return normalizeHaneulAddress(arg.Object.Shared.objectId);
}

export function getSharedObjectInput(
  arg: BuilderCallArg,
): SharedObjectRef | undefined {
  return typeof arg === 'object' && 'Object' in arg && 'Shared' in arg.Object
    ? arg.Object.Shared
    : undefined;
}

export function isSharedObjectInput(arg: BuilderCallArg): boolean {
  return !!getSharedObjectInput(arg);
}

export function isMutableSharedObjectInput(arg: BuilderCallArg): boolean {
  return getSharedObjectInput(arg)?.mutable ?? false;
}
