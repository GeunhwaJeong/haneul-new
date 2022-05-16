// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

/** Base64 string representing the object digest */
export type TransactionDigest = string;
export type HaneulAddress = string;
export type ObjectOwner =
  | { AddressOwner: HaneulAddress }
  | { ObjectOwner: HaneulAddress }
  | 'Shared'
  | 'Immutable';
