// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Provider } from './provider';
import { JsonRpcClient } from '../rpc/client';
import {
  isGetObjectDataResponse,
  isGetOwnedObjectsResponse,
  isGetTxnDigestsResponse,
  isTransactionEffectsResponse,
  isTransactionResponse,
} from '../index.guard';
import {
  GatewayTxSeqNumber,
  GetTxnDigestsResponse,
  GetObjectDataResponse,
  HaneulObjectInfo,
  TransactionDigest,
  TransactionEffectsResponse,
  TransactionResponse,
  HaneulObjectRef,
  getObjectReference,
  Coin,
} from '../types';

const isNumber = (val: any): val is number => typeof val === 'number';
const isAny = (_val: any): _val is any => true;

export class JsonRpcProvider extends Provider {
  private client: JsonRpcClient;

  /**
   * Establish a connection to a Haneul Gateway endpoint
   *
   * @param endpoint URL to the Haneul Gateway endpoint
   */
  constructor(public endpoint: string) {
    super();
    this.client = new JsonRpcClient(endpoint);
  }

  // Objects
  async getObjectsOwnedByAddress(address: string): Promise<HaneulObjectInfo[]> {
    try {
      return await this.client.requestWithType(
        'haneul_getObjectsOwnedByAddress',
        [address],
        isGetOwnedObjectsResponse
      );
    } catch (err) {
      throw new Error(
        `Error fetching owned object: ${err} for address ${address}`
      );
    }
  }

  async getGasObjectsOwnedByAddress(address: string): Promise<HaneulObjectInfo[]> {
    const objects = await this.getObjectsOwnedByAddress(address);
    return objects.filter((obj: HaneulObjectInfo) => Coin.isHANEUL(obj));
  }

  async getObjectsOwnedByObject(objectId: string): Promise<HaneulObjectInfo[]> {
    try {
      return await this.client.requestWithType(
        'haneul_getObjectsOwnedByObject',
        [objectId],
        isGetOwnedObjectsResponse
      );
    } catch (err) {
      throw new Error(
        `Error fetching owned object: ${err} for objectId ${objectId}`
      );
    }
  }

  async getObject(objectId: string): Promise<GetObjectDataResponse> {
    try {
      return await this.client.requestWithType(
        'haneul_getObject',
        [objectId],
        isGetObjectDataResponse
      );
    } catch (err) {
      throw new Error(`Error fetching object info: ${err} for id ${objectId}`);
    }
  }

  async getObjectRef(objectId: string): Promise<HaneulObjectRef | undefined> {
    const resp = await this.getObject(objectId);
    return getObjectReference(resp);
  }

  async getObjectBatch(objectIds: string[]): Promise<GetObjectDataResponse[]> {
    const requests = objectIds.map(id => ({
      method: 'haneul_getObject',
      args: [id],
    }));
    try {
      return await this.client.batchRequestWithType(
        requests,
        isGetObjectDataResponse
      );
    } catch (err) {
      throw new Error(`Error fetching object info: ${err} for id ${objectIds}`);
    }
  }

  // Transactions

  async getTransactionsForObject(
    objectID: string
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactionsByInputObject',
        args: [objectID],
      },
      {
        method: 'haneul_getTransactionsByMutatedObject',
        args: [objectID],
      },
    ];

    try {
      const results = await this.client.batchRequestWithType(
        requests,
        isGetTxnDigestsResponse
      );
      return [...results[0], ...results[1]];
    } catch (err) {
      throw new Error(
        `Error getting transactions for object: ${err} for id ${objectID}`
      );
    }
  }

  async getTransactionsForAddress(
    addressID: string
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactionsToAddress',
        args: [addressID],
      },
      {
        method: 'haneul_getTransactionsFromAddress',
        args: [addressID],
      },
    ];

    try {
      const results = await this.client.batchRequestWithType(
        requests,
        isGetTxnDigestsResponse
      );
      return [...results[0], ...results[1]];
    } catch (err) {
      throw new Error(
        `Error getting transactions for address: ${err} for id ${addressID}`
      );
    }
  }

  async getTransactionWithEffects(
    digest: TransactionDigest
  ): Promise<TransactionEffectsResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getTransaction',
        [digest],
        isTransactionEffectsResponse
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error getting transaction with effects: ${err} for digest ${digest}`
      );
    }
  }

  async getTransactionWithEffectsBatch(
    digests: TransactionDigest[]
  ): Promise<TransactionEffectsResponse[]> {
    const requests = digests.map(d => ({
      method: 'haneul_getTransaction',
      args: [d],
    }));
    try {
      return await this.client.batchRequestWithType(
        requests,
        isTransactionEffectsResponse
      );
    } catch (err) {
      const list = digests.join(', ').substring(0, -2);
      throw new Error(
        `Error getting transaction effects: ${err} for digests [${list}]`
      );
    }
  }

  async executeTransaction(
    txnBytes: string,
    flag: string,
    signature: string,
    pubkey: string
  ): Promise<TransactionResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_executeTransaction',
        [txnBytes, flag, signature, pubkey],
        isTransactionResponse
      );
      return resp;
    } catch (err) {
      throw new Error(`Error executing transaction: ${err}}`);
    }
  }

  async getTotalTransactionNumber(): Promise<number> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getTotalTransactionNumber',
        [],
        isNumber
      );
      return resp;
    } catch (err) {
      throw new Error(`Error fetching total transaction number: ${err}`);
    }
  }

  async getTransactionDigestsInRange(
    start: GatewayTxSeqNumber,
    end: GatewayTxSeqNumber
  ): Promise<GetTxnDigestsResponse> {
    try {
      return await this.client.requestWithType(
        'haneul_getTransactionsInRange',
        [start, end],
        isGetTxnDigestsResponse
      );
    } catch (err) {
      throw new Error(
        `Error fetching transaction digests in range: ${err} for range ${start}-${end}`
      );
    }
  }

  async getRecentTransactions(count: number): Promise<GetTxnDigestsResponse> {
    try {
      return await this.client.requestWithType(
        'haneul_getRecentTransactions',
        [count],
        isGetTxnDigestsResponse
      );
    } catch (err) {
      throw new Error(
        `Error fetching recent transactions: ${err} for count ${count}`
      );
    }
  }

  async syncAccountState(address: string): Promise<any> {
    try {
      return await this.client.requestWithType(
        'haneul_syncAccountState',
        [address],
        isAny,
      );
    } catch (err) {
      throw new Error(
        `Error sync account address for address: ${address} with error: ${err}`,
      );
    }
  }
}
