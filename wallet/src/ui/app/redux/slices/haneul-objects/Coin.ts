// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { isHaneulMoveObject } from '@haneullabs/haneul.js';

import type { HaneulObject, HaneulMoveObject } from '@haneullabs/haneul.js';

const COIN_TYPE = '0x2::Coin::Coin';
const COIN_TYPE_ARG_REGEX = /^0x2::Coin::Coin<(.+)>$/;

// TODO use sdk
export class Coin {
    public static isCoin(obj: HaneulObject) {
        return isHaneulMoveObject(obj.data) && obj.data.type.startsWith(COIN_TYPE);
    }

    public static getCoinTypeArg(obj: HaneulMoveObject) {
        const res = obj.type.match(COIN_TYPE_ARG_REGEX);
        return res ? res[1] : null;
    }

    public static getCoinSymbol(coinTypeArg: string) {
        return coinTypeArg.substring(coinTypeArg.lastIndexOf(':') + 1);
    }

    public static getBalance(obj: HaneulMoveObject) {
        return BigInt(obj.fields.balance);
    }
}
