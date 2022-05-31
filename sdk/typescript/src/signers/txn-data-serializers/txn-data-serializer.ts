// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Base64DataBuffer } from '../../serialization/base64';
import { ObjectId, HaneulAddress, HaneulJsonValue } from '../../types';

///////////////////////////////
// Exported Types
export interface TransferCoinTransaction {
  objectId: ObjectId;
  gasPayment?: ObjectId;
  gasBudget: number;
  recipient: HaneulAddress;
}

export interface MoveCallTransaction {
  packageObjectId: ObjectId;
  module: string;
  function: string;
  typeArguments: string[];
  arguments: HaneulJsonValue[];
  gasPayment?: ObjectId;
  gasBudget: number;
}

///////////////////////////////
// Exported Abstracts
/**
 * Serializes a transaction to a string that can be signed by a `Signer`.
 */
export interface TxnDataSerializer {
  newTransferCoin(
    signerAddress: HaneulAddress,
    txn: TransferCoinTransaction
  ): Promise<Base64DataBuffer>;

  newMoveCall(
    signerAddress: HaneulAddress,
    txn: MoveCallTransaction
  ): Promise<Base64DataBuffer>;

  // TODO: add `newSplitCoin` and `newMergeCoin`
}
