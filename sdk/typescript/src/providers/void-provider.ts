// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { PublicKey, SignatureScheme } from '../cryptography/publickey';
import { HttpHeaders } from '../rpc/client';
import { Base64DataBuffer } from '../serialization/base64';
import { RawMoveCall } from '../signers/txn-data-serializers/txn-data-serializer';
import {
  CertifiedTransaction,
  TransactionDigest,
  GetTxnDigestsResponse,
  GatewayTxSeqNumber,
  HaneulObjectInfo,
  GetObjectDataResponse,
  HaneulObjectRef,
  HaneulMoveFunctionArgTypes,
  HaneulMoveNormalizedFunction,
  HaneulMoveNormalizedStruct,
  HaneulMoveNormalizedModule,
  HaneulMoveNormalizedModules,
  HaneulEventFilter,
  HaneulEventEnvelope,
  SubscriptionId,
  ExecuteTransactionRequestType,
  HaneulExecuteTransactionResponse,
  HaneulAddress,
  ObjectId,
  TransactionQuery,
  PaginatedTransactionDigests,
  EventQuery,
  PaginatedEvents,
  EventId,
  RpcApiVersion,
  FaucetResponse,
  Order,
  TransactionEffects,
  CoinMetadata,
  DevInspectResults,
} from '../types';
import { Provider } from './provider';

export class VoidProvider extends Provider {
  // API Version
  async getRpcApiVersion(): Promise<RpcApiVersion | undefined> {
    throw this.newError('getRpcApiVersion');
  }

  getCoinMetadata(_coinType: string): Promise<CoinMetadata> {
    throw new Error('getCoinMetadata');
  }

  // Governance
  async getReferenceGasPrice(): Promise<number> {
    throw this.newError('getReferenceGasPrice');
  }

  // Faucet
  async requestHaneulFromFaucet(
    _recipient: HaneulAddress,
    _httpHeaders?: HttpHeaders
  ): Promise<FaucetResponse> {
    throw this.newError('requestHaneulFromFaucet');
  }

  // Objects
  async getObjectsOwnedByAddress(_address: string): Promise<HaneulObjectInfo[]> {
    throw this.newError('getObjectsOwnedByAddress');
  }

  async getGasObjectsOwnedByAddress(
    _address: string
  ): Promise<HaneulObjectInfo[]> {
    throw this.newError('getGasObjectsOwnedByAddress');
  }

  async getCoinBalancesOwnedByAddress(
    _address: string,
    _typeArg?: string
  ): Promise<GetObjectDataResponse[]> {
    throw this.newError('getCoinBalancesOwnedByAddress');
  }

  async selectCoinsWithBalanceGreaterThanOrEqual(
    _address: string,
    _amount: bigint,
    _typeArg: string,
    _exclude: ObjectId[] = []
  ): Promise<GetObjectDataResponse[]> {
    throw this.newError('selectCoinsWithBalanceGreaterThanOrEqual');
  }

  async selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
    _address: string,
    _amount: bigint,
    _typeArg: string,
    _exclude: ObjectId[]
  ): Promise<GetObjectDataResponse[]> {
    throw this.newError('selectCoinSetWithCombinedBalanceGreaterThanOrEqual');
  }

  async getObject(_objectId: string): Promise<GetObjectDataResponse> {
    throw this.newError('getObject');
  }

  async getObjectRef(_objectId: string): Promise<HaneulObjectRef | undefined> {
    throw this.newError('getObjectRef');
  }

  // Transactions
  async getTransaction(
    _digest: TransactionDigest
  ): Promise<CertifiedTransaction> {
    throw this.newError('getTransaction');
  }

  async executeTransaction(
    _txnBytes: Base64DataBuffer,
    _signatureScheme: SignatureScheme,
    _signature: Base64DataBuffer,
    _pubkey: PublicKey,
    _requestType: ExecuteTransactionRequestType
  ): Promise<HaneulExecuteTransactionResponse> {
    throw this.newError('executeTransaction with request Type');
  }

  devInspectTransaction(_txBytes: string): Promise<DevInspectResults> {
    throw this.newError('devInspectTransaction');
  }

  async devInspectMoveCall(
    _sender: HaneulAddress,
    _moveCall: RawMoveCall
  ): Promise<DevInspectResults> {
    throw this.newError('devInspectMoveCall');
  }

  dryRunTransaction(_txBytes: string): Promise<TransactionEffects> {
    throw this.newError('dryRunTransaction');
  }

  async getTotalTransactionNumber(): Promise<number> {
    throw this.newError('getTotalTransactionNumber');
  }

  async getTransactionDigestsInRange(
    _start: GatewayTxSeqNumber,
    _end: GatewayTxSeqNumber
  ): Promise<GetTxnDigestsResponse> {
    throw this.newError('getTransactionDigestsInRange');
  }

  async getMoveFunctionArgTypes(
    _objectId: string,
    _moduleName: string,
    _functionName: string
  ): Promise<HaneulMoveFunctionArgTypes> {
    throw this.newError('getMoveFunctionArgTypes');
  }

  async getNormalizedMoveModulesByPackage(
    _objectId: string
  ): Promise<HaneulMoveNormalizedModules> {
    throw this.newError('getNormalizedMoveModulesByPackage');
  }

  async getNormalizedMoveModule(
    _objectId: string,
    _moduleName: string
  ): Promise<HaneulMoveNormalizedModule> {
    throw this.newError('getNormalizedMoveModule');
  }

  async getNormalizedMoveFunction(
    _objectId: string,
    _moduleName: string,
    _functionName: string
  ): Promise<HaneulMoveNormalizedFunction> {
    throw this.newError('getNormalizedMoveFunction');
  }

  async getNormalizedMoveStruct(
    _objectId: string,
    _oduleName: string,
    _structName: string
  ): Promise<HaneulMoveNormalizedStruct> {
    throw this.newError('getNormalizedMoveStruct');
  }

  async syncAccountState(_address: string): Promise<any> {
    throw this.newError('syncAccountState');
  }

  async subscribeEvent(
    _filter: HaneulEventFilter,
    _onMessage: (event: HaneulEventEnvelope) => void
  ): Promise<SubscriptionId> {
    throw this.newError('subscribeEvent');
  }

  async unsubscribeEvent(_id: SubscriptionId): Promise<boolean> {
    throw this.newError('unsubscribeEvent');
  }

  private newError(operation: string): Error {
    return new Error(`Please use a valid provider for ${operation}`);
  }

  async getTransactions(
    _query: TransactionQuery,
    _cursor: TransactionDigest | null,
    _limit: number | null,
    _order: Order
  ): Promise<PaginatedTransactionDigests> {
    throw this.newError('getTransactions');
  }

  async getEvents(
    _query: EventQuery,
    _cursor: EventId | null,
    _limit: number | null,
    _order: Order
  ): Promise<PaginatedEvents> {
    throw this.newError('getEvents');
  }
}
