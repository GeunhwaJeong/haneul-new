// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

export type HaneulMoveFunctionArgType = string | { Object: string };

export type HaneulMoveNormalizedFunction = {
	visibility: HaneulMoveVisibility;
	isEntry: boolean;
	typeParameters: HaneulMoveAbilitySet[];
	parameters: HaneulMoveNormalizedType[];
	return: HaneulMoveNormalizedType[];
};

export type HaneulMoveNormalizedType =
	| string
	| HaneulMoveNormalizedTypeParameterType
	| { Reference: HaneulMoveNormalizedType }
	| { MutableReference: HaneulMoveNormalizedType }
	| { Vector: HaneulMoveNormalizedType }
	| HaneulMoveNormalizedStructType;

export type HaneulMoveNormalizedStructType = {
	Struct: {
		address: string;
		module: string;
		name: string;
		typeArguments: HaneulMoveNormalizedType[];
	};
};

export type HaneulMoveAbilitySet = {
	abilities: string[];
};

export type HaneulMoveNormalizedTypeParameterType = {
	TypeParameter: number;
};

export type HaneulMoveVisibility = 'Private' | 'Public' | 'Friend';

export type HaneulMoveNormalizedModule = {
	fileFormatVersion: number;
	address: string;
	name: string;
	friends: HaneulMoveModuleId[];
	structs: Record<string, HaneulMoveNormalizedStruct>;
	exposedFunctions: Record<string, HaneulMoveNormalizedFunction>;
};

export type HaneulMoveModuleId = {
	address: string;
	name: string;
};

export type HaneulMoveNormalizedModules = Record<string, HaneulMoveNormalizedModule>;

export type HaneulMoveNormalizedStruct = {
	abilities: HaneulMoveAbilitySet;
	typeParameters: HaneulMoveStructTypeParameter[];
	fields: HaneulMoveNormalizedField[];
};

export type HaneulMoveStructTypeParameter = {
	constraints: HaneulMoveAbilitySet;
	isPhantom: boolean;
};

export type HaneulMoveNormalizedField = {
	name: string;
	type: HaneulMoveNormalizedType;
};

export type HaneulMovePackage = {
	/** A mapping from module name to disassembled Move bytecode */
	disassembled: MovePackageContent;
};

export type MovePackageContent = Record<string, string>;

export type MoveCallMetrics = {
	rank3Days: MoveCallMetric[];
	rank7Days: MoveCallMetric[];
	rank30Days: MoveCallMetric[];
};

export type MoveCallMetric = [
	{
		module: string;
		package: string;
		function: string;
	},
	string,
];
