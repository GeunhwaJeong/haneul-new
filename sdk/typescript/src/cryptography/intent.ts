// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

// See: haneul/crates/haneul-types/src/intent.rs
export enum AppId {
	Haneul = 0,
}

export enum IntentVersion {
	V0 = 0,
}

export enum IntentScope {
	TransactionData = 0,
	TransactionEffects = 1,
	CheckpointSummary = 2,
	PersonalMessage = 3,
}

export type Intent = [IntentScope, IntentVersion, AppId];

function intentWithScope(scope: IntentScope): Intent {
	return [scope, IntentVersion.V0, AppId.Haneul];
}

export function messageWithIntent(scope: IntentScope, message: Uint8Array) {
	const intent = intentWithScope(scope);
	const intentMessage = new Uint8Array(intent.length + message.length);
	intentMessage.set(intent);
	intentMessage.set(message, intent.length);
	return intentMessage;
}
