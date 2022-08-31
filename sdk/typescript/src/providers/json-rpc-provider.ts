// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Provider } from './provider';
import { JsonRpcClient } from '../rpc/client';
import {
  isGetObjectDataResponse,
  isGetOwnedObjectsResponse,
  isGetTxnDigestsResponse,
  isHaneulTransactionResponse,
  isHaneulMoveFunctionArgTypes,
  isHaneulMoveNormalizedModules,
  isHaneulMoveNormalizedModule,
  isHaneulMoveNormalizedFunction,
  isHaneulMoveNormalizedStruct,
} from '../types/index.guard';
import {
  GatewayTxSeqNumber,
  GetTxnDigestsResponse,
  GetObjectDataResponse,
  HaneulObjectInfo,
  HaneulMoveFunctionArgTypes,
  HaneulMoveNormalizedModules,
  HaneulMoveNormalizedModule,
  HaneulMoveNormalizedFunction,
  HaneulMoveNormalizedStruct,
  TransactionDigest,
  HaneulTransactionResponse,
  HaneulObjectRef,
  getObjectReference,
  Coin,
} from '../types';
import { SignatureScheme } from '../cryptography/publickey';

const isNumber = (val: any): val is number => typeof val === 'number';
const isAny = (_val: any): _val is any => true;

export class JsonRpcProvider extends Provider {
  protected client: JsonRpcClient;
  /**
   * Establish a connection to a Haneul RPC endpoint
   *
   * @param endpoint URL to the Haneul RPC endpoint
   * @param skipDataValidation default to `false`. If set to `true`, the rpc
   * client will not check if the responses from the RPC server conform to the schema
   * defined in the TypeScript SDK. The mismatches often happen when the SDK
   * is in a different version than the RPC server. Skipping the validation
   * can maximize the version compatibility of the SDK, as not all the schema
   * changes in the RPC response will affect the caller, but the caller needs to
   * understand that the data may not match the TypeSrcript definitions.
   */
  constructor(
    public endpoint: string,
    public skipDataValidation: boolean = false
  ) {
    super();
    this.client = new JsonRpcClient(endpoint);
  }

  // Move info
  async getMoveFunctionArgTypes(
    objectId: string,
    moduleName: string,
    functionName: string
  ): Promise<HaneulMoveFunctionArgTypes> {
    try {
      return await this.client.requestWithType(
        'haneul_getMoveFunctionArgTypes',
        [objectId, moduleName, functionName],
        isHaneulMoveFunctionArgTypes,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching Move function arg types with package object ID: ${objectId}, module name: ${moduleName}, function name: ${functionName}`
      );
    }
  }

  async getNormalizedMoveModulesByPackage(
    objectId: string
  ): Promise<HaneulMoveNormalizedModules> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModulesByPackage',
        [objectId],
        isHaneulMoveNormalizedModules,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(`Error fetching package: ${err} for package ${objectId}`);
    }
  }

  async getNormalizedMoveModule(
    objectId: string,
    moduleName: string
  ): Promise<HaneulMoveNormalizedModule> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModule',
        [objectId, moduleName],
        isHaneulMoveNormalizedModule,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching module: ${err} for package ${objectId}, module ${moduleName}}`
      );
    }
  }

  async getNormalizedMoveFunction(
    objectId: string,
    moduleName: string,
    functionName: string
  ): Promise<HaneulMoveNormalizedFunction> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveFunction',
        [objectId, moduleName, functionName],
        isHaneulMoveNormalizedFunction,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching function: ${err} for package ${objectId}, module ${moduleName} and function ${functionName}}`
      );
    }
  }

  async getNormalizedMoveStruct(
    objectId: string,
    moduleName: string,
    structName: string
  ): Promise<HaneulMoveNormalizedStruct> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveStruct',
        [objectId, moduleName, structName],
        isHaneulMoveNormalizedStruct,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching struct: ${err} for package ${objectId}, module ${moduleName} and struct ${structName}}`
      );
    }
  }

  // Objects
  async getObjectsOwnedByAddress(address: string): Promise<HaneulObjectInfo[]> {
    try {
      return await this.client.requestWithType(
        'haneul_getObjectsOwnedByAddress',
        [address],
        isGetOwnedObjectsResponse,
        this.skipDataValidation
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
        isGetOwnedObjectsResponse,
        this.skipDataValidation
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
        isGetObjectDataResponse,
        this.skipDataValidation
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
        isGetObjectDataResponse,
        this.skipDataValidation
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
        isGetTxnDigestsResponse,
        this.skipDataValidation
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
        isGetTxnDigestsResponse,
        this.skipDataValidation
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
  ): Promise<HaneulTransactionResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getTransaction',
        [digest],
        isHaneulTransactionResponse,
        this.skipDataValidation
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
  ): Promise<HaneulTransactionResponse[]> {
    const requests = digests.map(d => ({
      method: 'haneul_getTransaction',
      args: [d],
    }));
    try {
      return await this.client.batchRequestWithType(
        requests,
        isHaneulTransactionResponse,
        this.skipDataValidation
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
    signatureScheme: SignatureScheme,
    signature: string,
    pubkey: string
  ): Promise<HaneulTransactionResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_executeTransaction',
        [txnBytes, signatureScheme, signature, pubkey],
        isHaneulTransactionResponse,
        this.skipDataValidation
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
        isNumber,
        this.skipDataValidation
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
        isGetTxnDigestsResponse,
        this.skipDataValidation
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
        isGetTxnDigestsResponse,
        this.skipDataValidation
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
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error sync account address for address: ${address} with error: ${err}`
      );
    }
  }
}
