// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Provider } from './provider';
import { JsonRpcClient } from '../rpc/client';
import {
  isGetObjectDataResponse,
  isGetOwnedObjectsResponse,
  isGetTxnDigestsResponse,
  isPaginatedTransactionDigests,
  isHaneulEvents,
  isHaneulExecuteTransactionResponse,
  isHaneulMoveFunctionArgTypes,
  isHaneulMoveNormalizedFunction,
  isHaneulMoveNormalizedModule,
  isHaneulMoveNormalizedModules,
  isHaneulMoveNormalizedStruct,
  isHaneulTransactionResponse,
} from '../types/index.guard';
import {
  Coin,
  DEFAULT_END_TIME,
  DEFAULT_START_TIME,
  EVENT_QUERY_MAX_LIMIT,
  ExecuteTransactionRequestType,
  CoinDenominationInfoResponse,
  GatewayTxSeqNumber,
  GetObjectDataResponse,
  getObjectReference,
  GetTxnDigestsResponse,
  ObjectId,
  ObjectOwner,
  Ordering,
  PaginatedTransactionDigests,
  SubscriptionId,
  HaneulAddress,
  HaneulEventEnvelope,
  HaneulEventFilter,
  HaneulEvents,
  HaneulExecuteTransactionResponse,
  HaneulMoveFunctionArgTypes,
  HaneulMoveNormalizedFunction,
  HaneulMoveNormalizedModule,
  HaneulMoveNormalizedModules,
  HaneulMoveNormalizedStruct,
  HaneulObjectInfo,
  HaneulObjectRef,
  HaneulTransactionResponse,
  TransactionDigest,
  TransactionQuery,
  HANEUL_TYPE_ARG,
  normalizeHaneulAddress,
} from '../types';
import { SignatureScheme } from '../cryptography/publickey';
import {
  DEFAULT_CLIENT_OPTIONS,
  WebsocketClient,
  WebsocketClientOptions,
} from '../rpc/websocket-client';

const isNumber = (val: any): val is number => typeof val === 'number';
const isAny = (_val: any): _val is any => true;

const PRE_PAGINATION_API_VERSION = '0.11.0';
export const LATEST_RPC_API_VERSION = 'latest';

export class JsonRpcProvider extends Provider {
  protected client: JsonRpcClient;
  protected wsClient: WebsocketClient;
  /**
   * Establish a connection to a Haneul RPC endpoint
   *
   * @param endpoint URL to the Haneul RPC endpoint
   * @param skipDataValidation default to `true`. If set to `false`, the rpc
   * client will throw an error if the responses from the RPC server do not
   * conform to the schema defined in the TypeScript SDK. If set to `true`, the
   * rpc client will log the mismatch as a warning message instead of throwing an
   * error. The mismatches often happen when the SDK is in a different version than
   * the RPC server. Skipping the validation can maximize
   * the version compatibility of the SDK, as not all the schema
   * changes in the RPC response will affect the caller, but the caller needs to
   * understand that the data may not match the TypeSrcript definitions.
   * @param rpcAPIVersion controls which type of RPC API version to use.
   */
  constructor(
    public endpoint: string,
    public skipDataValidation: boolean = true,
    // TODO: Update the default value after we deploy 0.12.0
    private rpcAPIVersion: string = PRE_PAGINATION_API_VERSION,
    public socketOptions: WebsocketClientOptions = DEFAULT_CLIENT_OPTIONS
  ) {
    super();

    this.client = new JsonRpcClient(endpoint);
    this.wsClient = new WebsocketClient(
      endpoint,
      skipDataValidation,
      socketOptions
    );
  }

  async getRpcApiVersion(): Promise<string> {
    // TODO: we should fetch the API version from
    // the RPC endpoint instead
    return this.rpcAPIVersion;
  }

  // Move info
  async getMoveFunctionArgTypes(
    packageId: string,
    moduleName: string,
    functionName: string
  ): Promise<HaneulMoveFunctionArgTypes> {
    try {
      return await this.client.requestWithType(
        'haneul_getMoveFunctionArgTypes',
        [packageId, moduleName, functionName],
        isHaneulMoveFunctionArgTypes,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching Move function arg types with package object ID: ${packageId}, module name: ${moduleName}, function name: ${functionName}`
      );
    }
  }

  async getNormalizedMoveModulesByPackage(
    packageId: string
  ): Promise<HaneulMoveNormalizedModules> {
    // TODO: Add caching since package object does not change
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModulesByPackage',
        [packageId],
        isHaneulMoveNormalizedModules,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching package: ${err} for package ${packageId}`
      );
    }
  }

  async getNormalizedMoveModule(
    packageId: string,
    moduleName: string
  ): Promise<HaneulMoveNormalizedModule> {
    // TODO: Add caching since package object does not change
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModule',
        [packageId, moduleName],
        isHaneulMoveNormalizedModule,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching module: ${err} for package ${packageId}, module ${moduleName}}`
      );
    }
  }

  async getNormalizedMoveFunction(
    packageId: string,
    moduleName: string,
    functionName: string
  ): Promise<HaneulMoveNormalizedFunction> {
    // TODO: Add caching since package object does not change
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveFunction',
        [packageId, moduleName, functionName],
        isHaneulMoveNormalizedFunction,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching function: ${err} for package ${packageId}, module ${moduleName} and function ${functionName}}`
      );
    }
  }

  async getNormalizedMoveStruct(
    packageId: string,
    moduleName: string,
    structName: string
  ): Promise<HaneulMoveNormalizedStruct> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveStruct',
        [packageId, moduleName, structName],
        isHaneulMoveNormalizedStruct,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error fetching struct: ${err} for package ${packageId}, module ${moduleName} and struct ${structName}}`
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

  getCoinDenominationInfo(coinType: string): CoinDenominationInfoResponse {
    const [packageId, module, symbol] = coinType.split('::');
    if (
      normalizeHaneulAddress(packageId) !== normalizeHaneulAddress('0x2') ||
      module != 'haneul' ||
      symbol !== 'HANEUL'
    ) {
      throw new Error(
        'only HANEUL coin is supported in getCoinDenominationInfo for now.'
      );
    }

    return {
      coinType: coinType,
      basicUnit: 'GEUNHWA',
      decimalNumber: 9,
    };
  }

  async getCoinBalancesOwnedByAddress(
    address: string,
    typeArg?: string
  ): Promise<GetObjectDataResponse[]> {
    const objects = await this.getObjectsOwnedByAddress(address);
    const coinIds = objects
      .filter(
        (obj: HaneulObjectInfo) =>
          Coin.isCoin(obj) &&
          (typeArg === undefined || typeArg === Coin.getCoinTypeArg(obj))
      )
      .map((c) => c.objectId);

    return await this.getObjectBatch(coinIds);
  }

  async selectCoinsWithBalanceGreaterThanOrEqual(
    address: string,
    amount: bigint,
    typeArg: string = HANEUL_TYPE_ARG,
    exclude: ObjectId[] = []
  ): Promise<GetObjectDataResponse[]> {
    const coins = await this.getCoinBalancesOwnedByAddress(address, typeArg);
    return (await Coin.selectCoinsWithBalanceGreaterThanOrEqual(
      coins,
      amount,
      exclude
    )) as GetObjectDataResponse[];
  }

  async selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
    address: string,
    amount: bigint,
    typeArg: string = HANEUL_TYPE_ARG,
    exclude: ObjectId[] = []
  ): Promise<GetObjectDataResponse[]> {
    const coins = await this.getCoinBalancesOwnedByAddress(address, typeArg);
    return (await Coin.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
      coins,
      amount,
      exclude
    )) as GetObjectDataResponse[];
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
    const requests = objectIds.map((id) => ({
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
  async getTransactions(
    query: TransactionQuery,
    cursor: TransactionDigest | null,
    limit: number | null,
    order: Ordering
  ): Promise<PaginatedTransactionDigests> {
    try {
      return await this.client.requestWithType(
        'haneul_getTransactions',
        [query, cursor, limit, order],
        isPaginatedTransactionDigests,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting transactions for query: ${err} for query ${query}`
      );
    }
  }

  async getTransactionsForObject(
    objectID: string,
    ordering: Ordering = 'Descending'
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactions',
        args: [{ InputObject: objectID }, null, null, ordering],
      },
      {
        method: 'haneul_getTransactions',
        args: [{ MutatedObject: objectID }, null, null, ordering],
      },
    ];

    try {
      const results = await this.client.batchRequestWithType(
        requests,
        isPaginatedTransactionDigests,
        this.skipDataValidation
      );
      return [...results[0].data, ...results[1].data];
    } catch (err) {
      throw new Error(
        `Error getting transactions for object: ${err} for id ${objectID}`
      );
    }
  }

  async getTransactionsForAddress(
    addressID: string,
    ordering: Ordering = 'Descending'
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactions',
        args: [{ ToAddress: addressID }, null, null, ordering],
      },
      {
        method: 'haneul_getTransactions',
        args: [{ FromAddress: addressID }, null, null, ordering],
      },
    ];
    try {
      const results = await this.client.batchRequestWithType(
        requests,
        isPaginatedTransactionDigests,
        this.skipDataValidation
      );
      return [...results[0].data, ...results[1].data];
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
    const requests = digests.map((d) => ({
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

  async executeTransactionWithRequestType(
    txnBytes: string,
    signatureScheme: SignatureScheme,
    signature: string,
    pubkey: string,
    requestType: ExecuteTransactionRequestType = 'WaitForEffectsCert'
  ): Promise<HaneulExecuteTransactionResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_executeTransaction',
        [txnBytes, signatureScheme, signature, pubkey, requestType],
        isHaneulExecuteTransactionResponse,
        this.skipDataValidation
      );
      return resp;
    } catch (err) {
      throw new Error(`Error executing transaction with request type: ${err}}`);
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

  // Events

  async getEventsByTransaction(
    digest: TransactionDigest,
    count: number = EVENT_QUERY_MAX_LIMIT
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByTransaction',
        [digest, count],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by transaction: ${digest}, with error: ${err}`
      );
    }
  }

  async getEventsByModule(
    package_: string,
    module: string,
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByModule',
        [package_, module, count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by transaction module: ${package_}::${module}, with error: ${err}`
      );
    }
  }

  async getEventsByMoveEventStructName(
    moveEventStructName: string,
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByMoveEventStructName',
        [moveEventStructName, count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by move event struct name: ${moveEventStructName}, with error: ${err}`
      );
    }
  }

  async getEventsBySender(
    sender: HaneulAddress,
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsBySender',
        [sender, count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by sender: ${sender}, with error: ${err}`
      );
    }
  }

  async getEventsByRecipient(
    recipient: ObjectOwner,
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByRecipient',
        [recipient, count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by receipient: ${recipient}, with error: ${err}`
      );
    }
  }

  async getEventsByObject(
    object: ObjectId,
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByObject',
        [object, count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by object: ${object}, with error: ${err}`
      );
    }
  }

  async getEventsByTimeRange(
    count: number = EVENT_QUERY_MAX_LIMIT,
    startTime: number = DEFAULT_START_TIME,
    endTime: number = DEFAULT_END_TIME
  ): Promise<HaneulEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEventsByTimeRange',
        [count, startTime, endTime],
        isHaneulEvents,
        this.skipDataValidation
      );
    } catch (err) {
      throw new Error(
        `Error getting events by time range: ${startTime} thru ${endTime}, with error: ${err}`
      );
    }
  }

  async subscribeEvent(
    filter: HaneulEventFilter,
    onMessage: (event: HaneulEventEnvelope) => void
  ): Promise<SubscriptionId> {
    return this.wsClient.subscribeEvent(filter, onMessage);
  }

  async unsubscribeEvent(id: SubscriptionId): Promise<boolean> {
    return this.wsClient.unsubscribeEvent(id);
  }
}
