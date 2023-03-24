// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { describe, it, expect, beforeAll } from 'vitest';
import {
  getCreatedObjects,
  getObjectId,
  getSharedObjectInitialVersion,
  isMutableSharedObjectInput,
  isSharedObjectInput,
  ObjectId,
  HaneulObjectData,
  HaneulTransactionResponse,
  HANEUL_SYSTEM_STATE_OBJECT_ID,
  Transaction,
} from '../../src';
import { TransactionDataBuilder } from '../../src/builder/TransactionData';
import { publishPackage, setup, TestToolbox } from './utils/setup';

describe('Transaction Serialization and deserialization', () => {
  let toolbox: TestToolbox;
  let packageId: ObjectId;
  let publishTxn: HaneulTransactionResponse;
  let sharedObjectId: ObjectId;

  beforeAll(async () => {
    toolbox = await setup();
    const packagePath = __dirname + '/./data/serializer';
    ({ packageId, publishTxn } = await publishPackage(packagePath));
    const sharedObject = getCreatedObjects(publishTxn)!.filter(
      (o) => getSharedObjectInitialVersion(o.owner) !== undefined,
    )[0];
    sharedObjectId = getObjectId(sharedObject);
  });

  async function serializeAndDeserialize(tx: Transaction, mutable: boolean[]) {
    tx.setSender(await toolbox.address());
    const transactionBytes = await tx.build({ provider: toolbox.provider });
    const deserializedTxnBuilder =
      TransactionDataBuilder.fromBytes(transactionBytes);
    expect(
      deserializedTxnBuilder.inputs
        .filter((i) => isSharedObjectInput(i.value))
        .map((i) => isMutableSharedObjectInput(i.value)),
    ).toStrictEqual(mutable);
    const reserializedTxnBytes = await deserializedTxnBuilder.build();
    expect(reserializedTxnBytes).toEqual(transactionBytes);
  }

  it('Move Shared Object Call with mutable reference', async () => {
    const coins = await toolbox.getGasObjectsOwnedByAddress();

    const [{ haneulAddress: validatorAddress }] =
      await toolbox.getActiveValidators();

    const tx = new Transaction();
    const coin = coins[2].data as HaneulObjectData;
    tx.moveCall({
      target: '0x3::haneul_system::request_add_stake',
      arguments: [
        tx.object(HANEUL_SYSTEM_STATE_OBJECT_ID),
        tx.object(coin.objectId),
        tx.pure(validatorAddress),
      ],
    });
    await serializeAndDeserialize(tx, [true]);
  });

  it('Move Shared Object Call with immutable reference', async () => {
    const tx = new Transaction();
    tx.moveCall({
      target: `${packageId}::serializer_tests::value`,
      arguments: [tx.object(sharedObjectId)],
    });
    await serializeAndDeserialize(tx, [false]);
  });

  it('Move Shared Object Call with mixed usage of mutable and immutable references', async () => {
    const tx = new Transaction();
    tx.moveCall({
      target: `${packageId}::serializer_tests::value`,
      arguments: [tx.object(sharedObjectId)],
    });
    tx.moveCall({
      target: `${packageId}::serializer_tests::set_value`,
      arguments: [tx.object(sharedObjectId)],
    });
    await serializeAndDeserialize(tx, [true]);
  });
});
