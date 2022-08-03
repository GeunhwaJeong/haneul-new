// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  getObjectFields,
  GetObjectDataResponse,
  HaneulMoveObject,
  HaneulObjectInfo,
} from './objects';

import { getMoveObjectType } from './objects';

import BN from 'bn.js';

const COIN_TYPE = '0x2::coin::Coin';
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
