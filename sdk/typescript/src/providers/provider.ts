// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SignatureScheme } from '../cryptography/publickey';
import {
  GetObjectDataResponse,
  HaneulObjectInfo,
  GatewayTxSeqNumber,
  GetTxnDigestsResponse,
  TransactionResponse,
  HaneulObjectRef,
} from '../types';

///////////////////////////////
// Exported Abstracts
export abstract class Provider {
  // Objects
  /**
   * Get all objects owned by an address
   */
  abstract getObjectsOwnedByAddress(
    addressOrObjectId: string
  ): Promise<HaneulObjectInfo[]>;

  /**
   * Convenience method for getting all gas objects(HANEUL Tokens) owned by an address
   */
  abstract getGasObjectsOwnedByAddress(
    _address: string
  ): Promise<HaneulObjectInfo[]>;

  /**
   * Get details about an object
   */
  abstract getObject(objectId: string): Promise<GetObjectDataResponse>;

  /**
   * Get object reference(id, tx digest, version id)
   * @param objectId
   */
  abstract getObjectRef(objectId: string): Promise<HaneulObjectRef | undefined>;

  // Transactions
  /**
   * Get transaction digests for a given range
   *
   * NOTE: this method may get deprecated after DevNet
   */
  abstract getTransactionDigestsInRange(
    start: GatewayTxSeqNumber,
    end: GatewayTxSeqNumber
  ): Promise<GetTxnDigestsResponse>;

  /**
   * Get the latest `count` transactions
   *
   * NOTE: this method may get deprecated after DevNet
   */
  abstract getRecentTransactions(count: number): Promise<GetTxnDigestsResponse>;

  /**
   * Get total number of transactions
   * NOTE: this method may get deprecated after DevNet
   */
  abstract getTotalTransactionNumber(): Promise<number>;

  abstract executeTransaction(
    txnBytes: string,
    signatureScheme: SignatureScheme,
    signature: string,
    pubkey: string
  ): Promise<TransactionResponse>;

  abstract syncAccountState(address: string): Promise<any>;
  // TODO: add more interface methods
}
