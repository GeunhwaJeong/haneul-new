// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  getObjectFields,
  GetObjectDataResponse,
  HaneulMoveObject,
  HaneulObjectInfo,
  HaneulObject,
  HaneulData,
  getMoveObjectType,
} from './objects';
import { normalizeHaneulObjectId, HaneulAddress } from './common';

import BN from 'bn.js';
import { getOption, Option } from './option';
import { StructTag } from './haneul-bcs';

export const COIN_PACKAGE_ID = '0x2';
export const COIN_MODULE_NAME = 'coin';
export const COIN_TYPE = `${COIN_PACKAGE_ID}::${COIN_MODULE_NAME}::Coin`;
export const COIN_SPLIT_VEC_FUNC_NAME = 'split_vec';
export const COIN_JOIN_FUNC_NAME = 'join';
const COIN_TYPE_ARG_REGEX = /^0x2::coin::Coin<(.+)>$/;

type ObjectData = GetObjectDataResponse | HaneulMoveObject | HaneulObjectInfo;

/**
 * Utility class for 0x2::coin
 * as defined in https://github.com/GeunhwaJeong/haneul/blob/ca9046fd8b1a9e8634a4b74b0e7dabdc7ea54475/haneul_programmability/framework/sources/Coin.move#L4
 */
export class Coin {
  static isCoin(data: ObjectData): boolean {
    return Coin.getType(data)?.startsWith(COIN_TYPE) ?? false;
  }

  static getCoinTypeArg(obj: ObjectData) {
    const res = Coin.getType(obj)?.match(COIN_TYPE_ARG_REGEX);
    return res ? res[1] : null;
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

  static getBalance(
    data: GetObjectDataResponse | HaneulMoveObject
  ): BN | undefined {
    if (!Coin.isCoin(data)) {
      return undefined;
    }
    const balance = getObjectFields(data)?.balance;
    return new BN.BN(balance, 10);
  }

  static getZero(): BN {
    return new BN.BN('0', 10);
  }

  private static getType(data: ObjectData): string | undefined {
    if ('status' in data) {
      return getMoveObjectType(data);
    }
    return data.type;
  }
}

export type DelegationData = HaneulMoveObject &
  Pick<HaneulData, 'dataType'> & {
    type: '0x2::delegation::Delegation';
    fields: {
      active_delegation: Option<number>;
      delegate_amount: number;
      next_reward_unclaimed_epoch: number;
      validator_address: HaneulAddress;
      info: {
        id: string;
        version: number;
      };
      coin_locked_until_epoch: Option<HaneulMoveObject>;
      ending_epoch: Option<number>;
    };
  };

export type DelegationHaneulObject = Omit<HaneulObject, 'data'> & {
  data: DelegationData;
};

// Class for delegation.move
// see https://github.com/GeunhwaJeong/fastnft/blob/161aa27fe7eb8ecf2866ec9eb192e768f25da768/crates/haneul-framework/sources/governance/delegation.move
export class Delegation {
  public static readonly HANEUL_OBJECT_TYPE = '0x2::delegation::Delegation';
  private haneulObject: DelegationHaneulObject;

  public static isDelegationHaneulObject(
    obj: HaneulObject
  ): obj is DelegationHaneulObject {
    return 'type' in obj.data && obj.data.type === Delegation.HANEUL_OBJECT_TYPE;
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
