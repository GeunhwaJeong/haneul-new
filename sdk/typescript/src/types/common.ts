// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

export type TransactionDigest = string;
export type HaneulAddress = string;
export type ObjectOwner =
  | { AddressOwner: HaneulAddress }
  | { ObjectOwner: HaneulAddress }
  | 'Shared'
  | 'Immutable';
