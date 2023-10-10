// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import type { Infer } from 'superstruct';
import { nullable, number, object, string } from 'superstruct';

import type { StructTag } from '../bcs/index.js';
import type { CoinStruct } from '../client/types/index.js';
import type {
	HaneulMoveObject,
	HaneulObjectData,
	HaneulObjectInfo,
	HaneulObjectResponse,
} from '../types/objects.js';
import { getObjectFields, getObjectId, getObjectType } from '../types/objects.js';
import type { Option } from '../types/option.js';
import { getOption } from '../types/option.js';
import { normalizeHaneulObjectId } from '../utils/haneul-types.js';

export const HANEUL_SYSTEM_ADDRESS = '0x3';
export const HANEUL_FRAMEWORK_ADDRESS = '0x2';
export const MOVE_STDLIB_ADDRESS = '0x1';
export const OBJECT_MODULE_NAME = 'object';
export const UID_STRUCT_NAME = 'UID';
export const ID_STRUCT_NAME = 'ID';
export const HANEUL_TYPE_ARG = `${HANEUL_FRAMEWORK_ADDRESS}::haneul::HANEUL`;
export const VALIDATORS_EVENTS_QUERY = '0x3::validator_set::ValidatorEpochInfoEventV2';

export const HANEUL_CLOCK_OBJECT_ID = normalizeHaneulObjectId('0x6');

// `haneul::pay` module is used for Coin management (split, join, join_and_transfer etc);
export const PAY_MODULE_NAME = 'pay';
export const PAY_SPLIT_COIN_VEC_FUNC_NAME = 'split_vec';
export const PAY_JOIN_COIN_FUNC_NAME = 'join';
export const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

type ObjectData = ObjectDataFull | HaneulObjectInfo;
type ObjectDataFull = HaneulObjectResponse | HaneulMoveObject;

export function isObjectDataFull(resp: ObjectData | ObjectDataFull): resp is HaneulObjectResponse {
	return !!(resp as HaneulObjectResponse).data || !!(resp as HaneulMoveObject).type;
}

export const CoinMetadataStruct = object({
	decimals: number(),
	name: string(),
	symbol: string(),
	description: string(),
	iconUrl: nullable(string()),
	id: nullable(string()),
});

export type CoinMetadata = Infer<typeof CoinMetadataStruct>;

/**
 * Utility class for 0x2::coin
 * as defined in https://github.com/GeunhwaJeong/haneul/blob/ca9046fd8b1a9e8634a4b74b0e7dabdc7ea54475/haneul_programmability/framework/sources/Coin.move#L4
 */
export class Coin {
	static isCoin(data: ObjectData): boolean {
		return Coin.getType(data)?.match(COIN_TYPE_ARG_REGEX) != null;
	}

	static getCoinType(type: string) {
		const [, res] = type.match(COIN_TYPE_ARG_REGEX) ?? [];
		return res || null;
	}

	static getCoinTypeArg(obj: ObjectData) {
		const type = Coin.getType(obj);
		return type ? Coin.getCoinType(type) : null;
	}

	static isHANEUL(obj: ObjectData) {
		const arg = Coin.getCoinTypeArg(obj);
		return arg ? Coin.getCoinSymbol(arg) === 'HANEUL' : false;
	}

	static getCoinSymbol(coinTypeArg: string) {
		return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
	}

	static getCoinStructTag(coinTypeArg: string): StructTag {
		return {
			address: normalizeHaneulObjectId(coinTypeArg.split('::')[0]),
			module: coinTypeArg.split('::')[1],
			name: coinTypeArg.split('::')[2],
			typeParams: [],
		};
	}

	public static getID(obj: ObjectData): string {
		if ('fields' in obj) {
			return obj.fields.id.id;
		}
		return getObjectId(obj);
	}

	static totalBalance(coins: CoinStruct[]): bigint {
		return coins.reduce(
			(partialSum, c) => partialSum + Coin.getBalanceFromCoinStruct(c),
			BigInt(0),
		);
	}

	/**
	 * Sort coin by balance in an ascending order
	 */
	static sortByBalance(coins: CoinStruct[]): CoinStruct[] {
		return [...coins].sort((a, b) =>
			Coin.getBalanceFromCoinStruct(a) < Coin.getBalanceFromCoinStruct(b)
				? -1
				: Coin.getBalanceFromCoinStruct(a) > Coin.getBalanceFromCoinStruct(b)
				? 1
				: 0,
		);
	}

	static getBalanceFromCoinStruct(coin: CoinStruct): bigint {
		return BigInt(coin.balance);
	}

	static getBalance(data: ObjectDataFull): bigint | undefined {
		if (!Coin.isCoin(data)) {
			return undefined;
		}
		const balance = getObjectFields(data)?.balance;
		return BigInt(balance);
	}

	private static getType(data: ObjectData): string | null | undefined {
		if (isObjectDataFull(data)) {
			return getObjectType(data);
		}
		return data.type;
	}
}

export type DelegationData = HaneulMoveObject & {
	dataType: 'moveObject';
	type: '0x2::delegation::Delegation';
	fields: {
		active_delegation: Option<number>;
		delegate_amount: number;
		next_reward_unclaimed_epoch: number;
		validator_address: string;
		info: {
			id: string;
			version: number;
		};
		ending_epoch: Option<number>;
	};
};

export type DelegationHaneulObject = Omit<HaneulObjectData, 'data'> & {
	data: DelegationData;
};

// Class for delegation.move
// see https://github.com/GeunhwaJeong/fastnft/blob/161aa27fe7eb8ecf2866ec9eb192e768f25da768/crates/haneul-framework/sources/governance/delegation.move
export class Delegation {
	public static readonly HANEUL_OBJECT_TYPE = '0x2::delegation::Delegation';
	private haneulObject: DelegationHaneulObject;

	public static isDelegationHaneulObject(obj: HaneulObjectData): obj is DelegationHaneulObject {
		return 'type' in obj && obj.type === Delegation.HANEUL_OBJECT_TYPE;
	}

	constructor(obj: DelegationHaneulObject) {
		this.haneulObject = obj;
	}

	public nextRewardUnclaimedEpoch() {
		return this.haneulObject.data.fields.next_reward_unclaimed_epoch;
	}

	public activeDelegation() {
		return BigInt(getOption(this.haneulObject.data.fields.active_delegation) || 0);
	}

	public delegateAmount() {
		return this.haneulObject.data.fields.delegate_amount;
	}

	public endingEpoch() {
		return getOption(this.haneulObject.data.fields.ending_epoch);
	}

	public validatorAddress() {
		return this.haneulObject.data.fields.validator_address;
	}

	public isActive() {
		return this.activeDelegation() > 0 && !this.endingEpoch();
	}

	public hasUnclaimedRewards(epoch: number) {
		return (
			this.nextRewardUnclaimedEpoch() <= epoch &&
			(this.isActive() || (this.endingEpoch() || 0) > epoch)
		);
	}
}
