// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { HaneulObjectChange } from './generated.js';

export type HaneulObjectChangePublished = Extract<HaneulObjectChange, { type: 'published' }>;
export type HaneulObjectChangeTransferred = Extract<HaneulObjectChange, { type: 'transferred' }>;
export type HaneulObjectChangeMutated = Extract<HaneulObjectChange, { type: 'mutated' }>;
export type HaneulObjectChangeDeleted = Extract<HaneulObjectChange, { type: 'deleted' }>;
export type HaneulObjectChangeWrapped = Extract<HaneulObjectChange, { type: 'wrapped' }>;
export type HaneulObjectChangeCreated = Extract<HaneulObjectChange, { type: 'created' }>;
