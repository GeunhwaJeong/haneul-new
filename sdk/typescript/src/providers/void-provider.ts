// Copyright (c) 2022, Haneul Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SignatureScheme } from '../cryptography/publickey';
import {
  CertifiedTransaction,
  TransactionDigest,
  GetTxnDigestsResponse,
  GatewayTxSeqNumber,
  HaneulObjectInfo,
  GetObjectDataResponse,
  HaneulTransactionResponse,
  HaneulObjectRef,
  HaneulMoveFunctionArgTypes,
  HaneulMoveNormalizedFunction,
  HaneulMoveNormalizedStruct,
  HaneulMoveNormalizedModule,
  HaneulMoveNormalizedModules,
} from '../types';
import { Provider } from './provider';

export class VoidProvider extends Provider {
  // Objects
  async getObjectsOwnedByAddress(_address: string): Promise<HaneulObjectInfo[]> {
    throw this.newError('getObjectsOwnedByAddress');
  }

  async getGasObjectsOwnedByAddress(
    _address: string
  ): Promise<HaneulObjectInfo[]> {
    throw this.newError('getGasObjectsOwnedByAddress');
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
    _txnBytes: string,
    _signatureScheme: SignatureScheme,
    _signature: string,
    _pubkey: string
  ): Promise<HaneulTransactionResponse> {
    throw this.newError('executeTransaction');
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

  async getRecentTransactions(_count: number): Promise<GetTxnDigestsResponse> {
    throw this.newError('getRecentTransactions');
  }

  async getMoveFunctionArgTypes(
    _objectId: string,
    _moduleName: string,
    _functionName: string
  ): Promise<HaneulMoveFunctionArgTypes> {
    throw this.newError('getMoveFunctionArgTypes');
  }

  async getNormalizedMoveModulesByPackage(_objectId: string,): Promise<HaneulMoveNormalizedModules> {
    throw this.newError('getNormalizedMoveModulesByPackage');
  }

  async getNormalizedMoveModule(
    _objectId: string,
    _moduleName: string,
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

  private newError(operation: string): Error {
    return new Error(`Please use a valid provider for ${operation}`);
  }
}
