// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Infer } from 'superstruct';
import { boolean, define, literal, nullable, object, record, string, union } from 'superstruct';

import type { CallArg } from '../bcs/index.js';

export const ObjectOwner = union([
	object({
		AddressOwner: string(),
	}),
	object({
		ObjectOwner: string(),
	}),
	object({
		Shared: object({
			initial_shared_version: nullable(string()),
		}),
	}),
	literal('Immutable'),
]);
export type ObjectOwner = Infer<typeof ObjectOwner>;

export type HaneulJsonValue = boolean | number | string | CallArg | Array<HaneulJsonValue>;
export const HaneulJsonValue = define<HaneulJsonValue>('HaneulJsonValue', () => true);

const ProtocolConfigValue = union([
	object({ u32: string() }),
	object({ u64: string() }),
	object({ f64: string() }),
]);
type ProtocolConfigValue = Infer<typeof ProtocolConfigValue>;

export const ProtocolConfig = object({
	attributes: record(string(), nullable(ProtocolConfigValue)),
	featureFlags: record(string(), boolean()),
	maxSupportedProtocolVersion: string(),
	minSupportedProtocolVersion: string(),
	protocolVersion: string(),
});
export type ProtocolConfig = Infer<typeof ProtocolConfig>;
