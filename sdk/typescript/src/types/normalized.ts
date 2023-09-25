// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Infer } from 'superstruct';
import {
	array,
	boolean,
	define,
	is,
	literal,
	number,
	object,
	record,
	string,
	tuple,
	union,
} from 'superstruct';

export type HaneulMoveFunctionArgTypesResponse = Infer<typeof HaneulMoveFunctionArgType>[];

export const HaneulMoveFunctionArgType = union([string(), object({ Object: string() })]);

export const HaneulMoveFunctionArgTypes = array(HaneulMoveFunctionArgType);
export type HaneulMoveFunctionArgTypes = Infer<typeof HaneulMoveFunctionArgTypes>;

export const HaneulMoveModuleId = object({
	address: string(),
	name: string(),
});
export type HaneulMoveModuleId = Infer<typeof HaneulMoveModuleId>;

export const HaneulMoveVisibility = union([literal('Private'), literal('Public'), literal('Friend')]);
export type HaneulMoveVisibility = Infer<typeof HaneulMoveVisibility>;

export const HaneulMoveAbilitySet = object({
	abilities: array(string()),
});
export type HaneulMoveAbilitySet = Infer<typeof HaneulMoveAbilitySet>;

export const HaneulMoveStructTypeParameter = object({
	constraints: HaneulMoveAbilitySet,
	isPhantom: boolean(),
});
export type HaneulMoveStructTypeParameter = Infer<typeof HaneulMoveStructTypeParameter>;

export const HaneulMoveNormalizedTypeParameterType = object({
	TypeParameter: number(),
});
export type HaneulMoveNormalizedTypeParameterType = Infer<typeof HaneulMoveNormalizedTypeParameterType>;

export type HaneulMoveNormalizedType =
	| string
	| HaneulMoveNormalizedTypeParameterType
	| { Reference: HaneulMoveNormalizedType }
	| { MutableReference: HaneulMoveNormalizedType }
	| { Vector: HaneulMoveNormalizedType }
	| HaneulMoveNormalizedStructType;

export const MoveCallMetric = tuple([
	object({
		module: string(),
		package: string(),
		function: string(),
	}),
	string(),
]);

export type MoveCallMetric = Infer<typeof MoveCallMetric>;

export const MoveCallMetrics = object({
	rank3Days: array(MoveCallMetric),
	rank7Days: array(MoveCallMetric),
	rank30Days: array(MoveCallMetric),
});

export type MoveCallMetrics = Infer<typeof MoveCallMetrics>;

function isHaneulMoveNormalizedType(value: unknown): value is HaneulMoveNormalizedType {
	if (!value) return false;
	if (typeof value === 'string') return true;
	if (is(value, HaneulMoveNormalizedTypeParameterType)) return true;
	if (isHaneulMoveNormalizedStructType(value)) return true;
	if (typeof value !== 'object') return false;

	const valueProperties = value as Record<string, unknown>;
	if (is(valueProperties.Reference, HaneulMoveNormalizedType)) return true;
	if (is(valueProperties.MutableReference, HaneulMoveNormalizedType)) return true;
	if (is(valueProperties.Vector, HaneulMoveNormalizedType)) return true;
	return false;
}

export const HaneulMoveNormalizedType = define<HaneulMoveNormalizedType>(
	'HaneulMoveNormalizedType',
	isHaneulMoveNormalizedType,
);

export type HaneulMoveNormalizedStructType = {
	Struct: {
		address: string;
		module: string;
		name: string;
		typeArguments: HaneulMoveNormalizedType[];
	};
};

function isHaneulMoveNormalizedStructType(value: unknown): value is HaneulMoveNormalizedStructType {
	if (!value || typeof value !== 'object') return false;

	const valueProperties = value as Record<string, unknown>;
	if (!valueProperties.Struct || typeof valueProperties.Struct !== 'object') return false;

	const structProperties = valueProperties.Struct as Record<string, unknown>;
	if (
		typeof structProperties.address !== 'string' ||
		typeof structProperties.module !== 'string' ||
		typeof structProperties.name !== 'string' ||
		!Array.isArray(structProperties.typeArguments) ||
		!structProperties.typeArguments.every((value) => isHaneulMoveNormalizedType(value))
	) {
		return false;
	}

	return true;
}

// NOTE: This type is recursive, so we need to manually implement it:
export const HaneulMoveNormalizedStructType = define<HaneulMoveNormalizedStructType>(
	'HaneulMoveNormalizedStructType',
	isHaneulMoveNormalizedStructType,
);

export const HaneulMoveNormalizedFunction = object({
	visibility: HaneulMoveVisibility,
	isEntry: boolean(),
	typeParameters: array(HaneulMoveAbilitySet),
	parameters: array(HaneulMoveNormalizedType),
	return: array(HaneulMoveNormalizedType),
});
export type HaneulMoveNormalizedFunction = Infer<typeof HaneulMoveNormalizedFunction>;

export const HaneulMoveNormalizedField = object({
	name: string(),
	type: HaneulMoveNormalizedType,
});
export type HaneulMoveNormalizedField = Infer<typeof HaneulMoveNormalizedField>;

export const HaneulMoveNormalizedStruct = object({
	abilities: HaneulMoveAbilitySet,
	typeParameters: array(HaneulMoveStructTypeParameter),
	fields: array(HaneulMoveNormalizedField),
});
export type HaneulMoveNormalizedStruct = Infer<typeof HaneulMoveNormalizedStruct>;

export const HaneulMoveNormalizedModule = object({
	fileFormatVersion: number(),
	address: string(),
	name: string(),
	friends: array(HaneulMoveModuleId),
	structs: record(string(), HaneulMoveNormalizedStruct),
	exposedFunctions: record(string(), HaneulMoveNormalizedFunction),
});
export type HaneulMoveNormalizedModule = Infer<typeof HaneulMoveNormalizedModule>;

export const HaneulMoveNormalizedModules = record(string(), HaneulMoveNormalizedModule);
export type HaneulMoveNormalizedModules = Infer<typeof HaneulMoveNormalizedModules>;

export function extractMutableReference(
	normalizedType: HaneulMoveNormalizedType,
): HaneulMoveNormalizedType | undefined {
	return typeof normalizedType === 'object' && 'MutableReference' in normalizedType
		? normalizedType.MutableReference
		: undefined;
}

export function extractReference(
	normalizedType: HaneulMoveNormalizedType,
): HaneulMoveNormalizedType | undefined {
	return typeof normalizedType === 'object' && 'Reference' in normalizedType
		? normalizedType.Reference
		: undefined;
}

export function extractStructTag(
	normalizedType: HaneulMoveNormalizedType,
): HaneulMoveNormalizedStructType | undefined {
	if (typeof normalizedType === 'object' && 'Struct' in normalizedType) {
		return normalizedType;
	}

	const ref = extractReference(normalizedType);
	const mutRef = extractMutableReference(normalizedType);

	if (typeof ref === 'object' && 'Struct' in ref) {
		return ref;
	}

	if (typeof mutRef === 'object' && 'Struct' in mutRef) {
		return mutRef;
	}
	return undefined;
}
