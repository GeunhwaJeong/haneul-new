// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Base64DataBuffer } from '../../serialization/base64';
import { ObjectId, HaneulAddress } from '../../types';

///////////////////////////////
// Exported Types
export interface TransferCoinTransaction {
  signer: HaneulAddress;
  objectId: ObjectId;
  gasPayment: ObjectId;
  gasBudget: number;
  recipient: HaneulAddress;
}

///////////////////////////////
// Exported Abstracts
/**
 * Serializes a transaction to a string that can be signed by a `Signer`.
 */
export interface TxnDataSerializer {
  newTransferCoin(txn: TransferCoinTransaction): Promise<Base64DataBuffer>;

  // TODO: add more interface methods
}
