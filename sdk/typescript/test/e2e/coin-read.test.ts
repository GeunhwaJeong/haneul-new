// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import {
  LocalTxnDataSerializer,
  RawSigner,
} from '../../src';

import { publishPackage, setup, TestToolbox } from './utils/setup';

describe.each([{ useLocalTxnBuilder: true }])(
    'CoinRead API',
    ({ useLocalTxnBuilder }) => {
      let toolbox: TestToolbox;
      let signer: RawSigner;
      let packageId: string;
      let testType: string;

      beforeAll(async () => {
        toolbox = await setup();
        signer = new RawSigner(
          toolbox.keypair,
          toolbox.provider,
          useLocalTxnBuilder
            ? new LocalTxnDataSerializer(toolbox.provider)
            : undefined
        );
        const packagePath = __dirname + '/./data/coin_metadata';
        packageId = await publishPackage(signer, useLocalTxnBuilder, packagePath);
        testType = packageId + '::test::TEST';
      });
    
      it("Get coins with/without type", async () => {
        const  haneulCoins = await toolbox.provider.getCoins(toolbox.address());
        expect(haneulCoins.data.length).toEqual(5);
        
        const testCoins = await toolbox.provider.getCoins(toolbox.address(), testType);
        expect(testCoins.data.length).toEqual(2);

        const allCoins = await toolbox.provider.getAllCoins(toolbox.address());
        expect(allCoins.data.length).toEqual(7);
        expect(allCoins.nextCursor).toBeNull();

        //test paging with limit
        const someHaneulCoins = await toolbox.provider.getCoins(toolbox.address(), null, null, 3);
        expect(someHaneulCoins.data.length).toEqual(3);
        expect(someHaneulCoins.nextCursor).toBeTruthy();
      });

      it("Get balance with/without type", async () => {
        const haneulBalance = await toolbox.provider.getBalance(toolbox.address());
        expect(haneulBalance.coinType).toEqual("0x2::haneul::HANEUL");
        expect(haneulBalance.coinObjectCount).toEqual(5);
        expect(haneulBalance.totalBalance).toBeGreaterThan(0);

        const testBalance = await toolbox.provider.getBalance(toolbox.address(), testType);
        expect(testBalance.coinType).toEqual(testType);
        expect(testBalance.coinObjectCount).toEqual(2);
        expect(testBalance.totalBalance).toEqual(11);

        const allBalances = await toolbox.provider.getAllBalances(toolbox.address());
        expect(allBalances.length).toEqual(2);
      });

      it("Get total supply", async () => {
        const testSupply = await toolbox.provider.getTotalSupply(testType);
        expect(testSupply.value).toEqual(11);
      })
    
    }
)
