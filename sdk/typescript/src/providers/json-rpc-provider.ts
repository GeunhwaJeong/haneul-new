// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { Provider } from './provider';
import { ErrorResponse, HttpHeaders, JsonRpcClient } from '../rpc/client';
import {
  Coin,
  ExecuteTransactionRequestType,
  GatewayTxSeqNumber,
  getObjectReference,
  GetTxnDigestsResponse,
  ObjectId,
  PaginatedTransactionDigests,
  SubscriptionId,
  HaneulAddress,
  HaneulEventEnvelope,
  HaneulEventFilter,
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
  RpcApiVersion,
  parseVersionFromString,
  EventQuery,
  EventId,
  PaginatedEvents,
  FaucetResponse,
  Order,
  DevInspectResults,
  CoinMetadata,
  isValidTransactionDigest,
  isValidHaneulAddress,
  isValidHaneulObjectId,
  normalizeHaneulAddress,
  normalizeHaneulObjectId,
  CoinMetadataStruct,
  PaginatedCoins,
  HaneulObjectResponse,
  GetOwnedObjectsResponse,
  DelegatedStake,
  CoinBalance,
  CoinSupply,
  CheckpointDigest,
  Checkpoint,
  CommitteeInfo,
  DryRunTransactionResponse,
  HaneulObjectDataOptions,
  HaneulSystemStateSummary,
  CoinStruct,
} from '../types';
import { DynamicFieldName, DynamicFieldPage } from '../types/dynamic_fields';
import {
  DEFAULT_CLIENT_OPTIONS,
  WebsocketClient,
  WebsocketClientOptions,
} from '../rpc/websocket-client';
import { requestHaneulFromFaucet } from '../rpc/faucet-client';
import { any, is, number, array } from 'superstruct';
import { UnserializedSignableTransaction } from '../signers/txn-data-serializers/txn-data-serializer';
import { LocalTxnDataSerializer } from '../signers/txn-data-serializers/local-txn-data-serializer';
import { toB64 } from '@haneullabs/bcs';
import { SerializedSignature } from '../cryptography/signature';
import { Connection, devnetConnection } from '../rpc/connection';
import { Transaction } from '../builder';

export const TARGETED_RPC_VERSION = '0.27.0';

/**
 * Configuration options for the JsonRpcProvider. If the value of a field is not provided,
 * value in `DEFAULT_OPTIONS` for that field will be used
 */
export type RpcProviderOptions = {
  /**
   * Default to `true`. If set to `false`, the rpc
   * client will throw an error if the responses from the RPC server do not
   * conform to the schema defined in the TypeScript SDK. If set to `true`, the
   * rpc client will log the mismatch as a warning message instead of throwing an
   * error. The mismatches often happen when the SDK is in a different version than
   * the RPC server. Skipping the validation can maximize
   * the version compatibility of the SDK, as not all the schema
   * changes in the RPC response will affect the caller, but the caller needs to
   * understand that the data may not match the TypeSrcript definitions.
   */
  skipDataValidation?: boolean;
  /**
   * Configuration options for the websocket connection
   * TODO: Move to connection.
   */
  socketOptions?: WebsocketClientOptions;
  /**
   * Cache timeout in seconds for the RPC API Version
   */
  versionCacheTimeoutInSeconds?: number;

  /** Allow defining a custom RPC client to use */
  rpcClient?: JsonRpcClient;

  /** Allow defining a custom websocket client to use */
  websocketClient?: WebsocketClient;
};

const DEFAULT_OPTIONS: RpcProviderOptions = {
  skipDataValidation: true,
  socketOptions: DEFAULT_CLIENT_OPTIONS,
  versionCacheTimeoutInSeconds: 600,
};

export class JsonRpcProvider extends Provider {
  public connection: Connection;
  protected client: JsonRpcClient;
  protected wsClient: WebsocketClient;
  private rpcApiVersion: RpcApiVersion | undefined;
  private cacheExpiry: number | undefined;
  /**
   * Establish a connection to a Haneul RPC endpoint
   *
   * @param connection The `Connection` object containing configuration for the network.
   * @param options configuration options for the provider
   */
  constructor(
    // TODO: Probably remove the default endpoint here:
    connection: Connection = devnetConnection,
    public options: RpcProviderOptions = DEFAULT_OPTIONS,
  ) {
    super();

    this.connection = connection;

    const opts = { ...DEFAULT_OPTIONS, ...options };
    this.options = opts;
    // TODO: add header for websocket request
    this.client = opts.rpcClient ?? new JsonRpcClient(this.connection.fullnode);

    this.wsClient =
      opts.websocketClient ??
      new WebsocketClient(
        this.connection.websocket,
        opts.skipDataValidation!,
        opts.socketOptions,
      );
  }

  async getRpcApiVersion(): Promise<RpcApiVersion | undefined> {
    if (
      this.rpcApiVersion &&
      this.cacheExpiry &&
      this.cacheExpiry <= Date.now()
    ) {
      return this.rpcApiVersion;
    }
    try {
      const resp = await this.client.requestWithType(
        'rpc.discover',
        [],
        any(),
        this.options.skipDataValidation,
      );
      this.rpcApiVersion = parseVersionFromString(resp.info.version);
      this.cacheExpiry =
        // Date.now() is in milliseconds, but the timeout is in seconds
        Date.now() + (this.options.versionCacheTimeoutInSeconds ?? 0) * 1000;
      return this.rpcApiVersion;
    } catch (err) {
      console.warn('Error fetching version number of the RPC API', err);
    }
    return undefined;
  }

  async requestHaneulFromFaucet(
    recipient: HaneulAddress,
    httpHeaders?: HttpHeaders,
  ): Promise<FaucetResponse> {
    if (!this.connection.faucet) {
      throw new Error('Faucet URL is not specified');
    }
    return requestHaneulFromFaucet(this.connection.faucet, recipient, httpHeaders);
  }

  // Coins
  async getCoins(
    owner: HaneulAddress,
    coinType: string | null = null,
    cursor: ObjectId | null = null,
    limit: number | null = null,
  ): Promise<PaginatedCoins> {
    try {
      if (!owner || !isValidHaneulAddress(normalizeHaneulAddress(owner))) {
        throw new Error('Invalid Haneul address');
      }
      return await this.client.requestWithType(
        'haneul_getCoins',
        [owner, coinType, cursor, limit],
        PaginatedCoins,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error getting coins for owner ${owner}: ${err}`);
    }
  }

  async getAllCoins(
    owner: HaneulAddress,
    cursor: ObjectId | null = null,
    limit: number | null = null,
  ): Promise<PaginatedCoins> {
    try {
      if (!owner || !isValidHaneulAddress(normalizeHaneulAddress(owner))) {
        throw new Error('Invalid Haneul address');
      }
      return await this.client.requestWithType(
        'haneul_getAllCoins',
        [owner, cursor, limit],
        PaginatedCoins,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error getting all coins for owner ${owner}: ${err}`);
    }
  }

  async getBalance(
    owner: HaneulAddress,
    coinType: string | null = null,
  ): Promise<CoinBalance> {
    try {
      if (!owner || !isValidHaneulAddress(normalizeHaneulAddress(owner))) {
        throw new Error('Invalid Haneul address');
      }
      return await this.client.requestWithType(
        'haneul_getBalance',
        [owner, coinType],
        CoinBalance,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error getting balance for coin type ${coinType} for owner ${owner}: ${err}`,
      );
    }
  }

  async getAllBalances(owner: HaneulAddress): Promise<CoinBalance[]> {
    try {
      if (!owner || !isValidHaneulAddress(normalizeHaneulAddress(owner))) {
        throw new Error('Invalid Haneul address');
      }
      return await this.client.requestWithType(
        'haneul_getAllBalances',
        [owner],
        array(CoinBalance),
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error getting all balances for owner ${owner}: ${err}`);
    }
  }

  async getCoinMetadata(coinType: string): Promise<CoinMetadata> {
    try {
      return await this.client.requestWithType(
        'haneul_getCoinMetadata',
        [coinType],
        CoinMetadataStruct,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error fetching CoinMetadata for ${coinType}: ${err}`);
    }
  }

  async getTotalSupply(coinType: string): Promise<CoinSupply> {
    try {
      return await this.client.requestWithType(
        'haneul_getTotalSupply',
        [coinType],
        CoinSupply,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching total supply for Coin type ${coinType}: ${err}`,
      );
    }
  }

  // RPC endpoint
  async call(endpoint: string, params: Array<any>): Promise<any> {
    try {
      const response = await this.client.request(endpoint, params);
      if (is(response, ErrorResponse)) {
        throw new Error(`RPC Error: ${response.error.message}`);
      }
      return response.result;
    } catch (err) {
      throw new Error(`Error calling RPC endpoint ${endpoint}: ${err}`);
    }
  }

  // Move info
  async getMoveFunctionArgTypes(
    packageId: string,
    moduleName: string,
    functionName: string,
  ): Promise<HaneulMoveFunctionArgTypes> {
    try {
      return await this.client.requestWithType(
        'haneul_getMoveFunctionArgTypes',
        [packageId, moduleName, functionName],
        HaneulMoveFunctionArgTypes,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching Move function arg types with package object ID: ${packageId}, module name: ${moduleName}, function name: ${functionName}`,
      );
    }
  }

  async getNormalizedMoveModulesByPackage(
    packageId: string,
  ): Promise<HaneulMoveNormalizedModules> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModulesByPackage',
        [packageId],
        HaneulMoveNormalizedModules,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching package: ${err} for package ${packageId}`,
      );
    }
  }

  async getNormalizedMoveModule(
    packageId: string,
    moduleName: string,
  ): Promise<HaneulMoveNormalizedModule> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveModule',
        [packageId, moduleName],
        HaneulMoveNormalizedModule,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching module: ${err} for package ${packageId}, module ${moduleName}`,
      );
    }
  }

  async getNormalizedMoveFunction(
    packageId: string,
    moduleName: string,
    functionName: string,
  ): Promise<HaneulMoveNormalizedFunction> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveFunction',
        [packageId, moduleName, functionName],
        HaneulMoveNormalizedFunction,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching function: ${err} for package ${packageId}, module ${moduleName} and function ${functionName}`,
      );
    }
  }

  async getNormalizedMoveStruct(
    packageId: string,
    moduleName: string,
    structName: string,
  ): Promise<HaneulMoveNormalizedStruct> {
    try {
      return await this.client.requestWithType(
        'haneul_getNormalizedMoveStruct',
        [packageId, moduleName, structName],
        HaneulMoveNormalizedStruct,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching struct: ${err} for package ${packageId}, module ${moduleName} and struct ${structName}`,
      );
    }
  }

  // Objects
  async getObjectsOwnedByAddress(
    address: HaneulAddress,
    typeFilter?: string,
  ): Promise<HaneulObjectInfo[]> {
    try {
      if (!address || !isValidHaneulAddress(normalizeHaneulAddress(address))) {
        throw new Error('Invalid Haneul address');
      }
      const objects = await this.client.requestWithType(
        'haneul_getObjectsOwnedByAddress',
        [address],
        GetOwnedObjectsResponse,
        this.options.skipDataValidation,
      );
      // TODO: remove this once we migrated to the new queryObject API
      if (typeFilter) {
        return objects.filter(
          (obj: HaneulObjectInfo) =>
            obj.type === typeFilter || obj.type.startsWith(typeFilter + '<'),
        );
      }
      return objects;
    } catch (err) {
      throw new Error(
        `Error fetching owned object: ${err} for address ${address}`,
      );
    }
  }

  async selectCoinsWithBalanceGreaterThanOrEqual(
    address: HaneulAddress,
    amount: bigint,
    typeArg: string = HANEUL_TYPE_ARG,
    exclude: ObjectId[] = [],
  ): Promise<CoinStruct[]> {
    const coinsStruct = await this.getCoins(address, typeArg);
    return Coin.selectCoinsWithBalanceGreaterThanOrEqual(
      coinsStruct.data,
      amount,
      exclude,
    );
  }

  async selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
    address: HaneulAddress,
    amount: bigint,
    typeArg: string = HANEUL_TYPE_ARG,
    exclude: ObjectId[] = [],
  ): Promise<CoinStruct[]> {
    const coinsStruct = await this.getCoins(address, typeArg);
    const coins = coinsStruct.data;

    return Coin.selectCoinSetWithCombinedBalanceGreaterThanOrEqual(
      coins,
      amount,
      exclude,
    );
  }

  async getObject(
    objectId: ObjectId,
    options?: HaneulObjectDataOptions,
  ): Promise<HaneulObjectResponse> {
    try {
      if (!objectId || !isValidHaneulObjectId(normalizeHaneulObjectId(objectId))) {
        throw new Error('Invalid Haneul Object id');
      }
      return await this.client.requestWithType(
        'haneul_getObject',
        [objectId, options],
        HaneulObjectResponse,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error fetching object info: ${err} for id ${objectId}`);
    }
  }

  async getObjectRef(objectId: ObjectId): Promise<HaneulObjectRef | undefined> {
    const resp = await this.getObject(objectId);
    return getObjectReference(resp);
  }

  async getObjectBatch(
    objectIds: ObjectId[],
    options?: HaneulObjectDataOptions,
  ): Promise<HaneulObjectResponse[]> {
    try {
      const requests = objectIds.map((id) => {
        if (!id || !isValidHaneulObjectId(normalizeHaneulObjectId(id))) {
          throw new Error(`Invalid Haneul Object id ${id}`);
        }
        return {
          method: 'haneul_getObject',
          args: [id, options],
        };
      });
      return await this.client.batchRequestWithType(
        requests,
        HaneulObjectResponse,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching object info: ${err} for ids [${objectIds}]`,
      );
    }
  }

  // Transactions
  async getTransactions(
    query: TransactionQuery,
    cursor: TransactionDigest | null = null,
    limit: number | null = null,
    order: Order = 'descending',
  ): Promise<PaginatedTransactionDigests> {
    try {
      return await this.client.requestWithType(
        'haneul_getTransactions',
        [query, cursor, limit, order === 'descending'],
        PaginatedTransactionDigests,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error getting transactions for query: ${err} for query ${query}`,
      );
    }
  }

  async getTransactionsForObject(
    objectID: ObjectId,
    descendingOrder: boolean = true,
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactions',
        args: [{ InputObject: objectID }, null, null, descendingOrder],
      },
      {
        method: 'haneul_getTransactions',
        args: [{ MutatedObject: objectID }, null, null, descendingOrder],
      },
    ];

    try {
      if (!objectID || !isValidHaneulObjectId(normalizeHaneulObjectId(objectID))) {
        throw new Error('Invalid Haneul Object id');
      }
      const results = await this.client.batchRequestWithType(
        requests,
        PaginatedTransactionDigests,
        this.options.skipDataValidation,
      );
      return [...results[0].data, ...results[1].data];
    } catch (err) {
      throw new Error(
        `Error getting transactions for object: ${err} for id ${objectID}`,
      );
    }
  }

  async getTransactionsForAddress(
    addressID: HaneulAddress,
    descendingOrder: boolean = true,
  ): Promise<GetTxnDigestsResponse> {
    const requests = [
      {
        method: 'haneul_getTransactions',
        args: [{ ToAddress: addressID }, null, null, descendingOrder],
      },
      {
        method: 'haneul_getTransactions',
        args: [{ FromAddress: addressID }, null, null, descendingOrder],
      },
    ];
    try {
      if (!addressID || !isValidHaneulAddress(normalizeHaneulAddress(addressID))) {
        throw new Error('Invalid Haneul address');
      }
      const results = await this.client.batchRequestWithType(
        requests,
        PaginatedTransactionDigests,
        this.options.skipDataValidation,
      );
      return [...results[0].data, ...results[1].data];
    } catch (err) {
      throw new Error(
        `Error getting transactions for address: ${err} for id ${addressID}`,
      );
    }
  }

  async getTransactionWithEffects(
    digest: TransactionDigest,
  ): Promise<HaneulTransactionResponse> {
    try {
      if (!isValidTransactionDigest(digest)) {
        throw new Error('Invalid Transaction digest');
      }
      const resp = await this.client.requestWithType(
        'haneul_getTransaction',
        [digest],
        HaneulTransactionResponse,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error getting transaction with effects: ${err} for digest ${digest}`,
      );
    }
  }

  async getTransactionWithEffectsBatch(
    digests: TransactionDigest[],
  ): Promise<HaneulTransactionResponse[]> {
    try {
      const requests = digests.map((d) => {
        if (!isValidTransactionDigest(d)) {
          throw new Error(`Invalid Transaction digest ${d}`);
        }
        return {
          method: 'haneul_getTransaction',
          args: [d],
        };
      });
      return await this.client.batchRequestWithType(
        requests,
        HaneulTransactionResponse,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error getting transaction effects: ${err} for digests [${digests}]`,
      );
    }
  }

  async executeTransaction(
    txnBytes: Uint8Array | string,
    signature: SerializedSignature,
    requestType: ExecuteTransactionRequestType = 'WaitForEffectsCert',
  ): Promise<HaneulTransactionResponse> {
    try {
      return await this.client.requestWithType(
        'haneul_executeTransactionSerializedSig',
        [
          typeof txnBytes === 'string' ? txnBytes : toB64(txnBytes),
          signature,
          requestType,
        ],
        HaneulTransactionResponse,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error executing transaction with request type: ${err}`);
    }
  }

  async getTotalTransactionNumber(): Promise<number> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getTotalTransactionNumber',
        [],
        number(),
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(`Error fetching total transaction number: ${err}`);
    }
  }

  async getTransactionDigestsInRange(
    start: GatewayTxSeqNumber,
    end: GatewayTxSeqNumber,
  ): Promise<GetTxnDigestsResponse> {
    try {
      return await this.client.requestWithType(
        'haneul_getTransactionsInRange',
        [start, end],
        GetTxnDigestsResponse,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error fetching transaction digests in range: ${err} for range ${start}-${end}`,
      );
    }
  }

  // Governance
  async getReferenceGasPrice(): Promise<number> {
    try {
      return await this.client.requestWithType(
        'haneul_getReferenceGasPrice',
        [],
        number(),
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(`Error getting the reference gas price ${err}`);
    }
  }

  async getDelegatedStakes(address: HaneulAddress): Promise<DelegatedStake[]> {
    try {
      if (!address || !isValidHaneulAddress(normalizeHaneulAddress(address))) {
        throw new Error('Invalid Haneul address');
      }
      const resp = await this.client.requestWithType(
        'haneul_getDelegatedStakes',
        [address],
        array(DelegatedStake),
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(`Error in getDelegatedStake: ${err}`);
    }
  }

  async getLatestHaneulSystemState(): Promise<HaneulSystemStateSummary> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getLatestHaneulSystemState',
        [],
        HaneulSystemStateSummary,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(`Error in getLatestHaneulSystemState: ${err}`);
    }
  }

  // Events
  async getEvents(
    query: EventQuery,
    cursor: EventId | null,
    limit: number | null,
    order: Order = 'descending',
  ): Promise<PaginatedEvents> {
    try {
      return await this.client.requestWithType(
        'haneul_getEvents',
        [query, cursor, limit, order === 'descending'],
        PaginatedEvents,
        this.options.skipDataValidation,
      );
    } catch (err) {
      throw new Error(
        `Error getting events for query: ${err} for query ${query}`,
      );
    }
  }

  async subscribeEvent(
    filter: HaneulEventFilter,
    onMessage: (event: HaneulEventEnvelope) => void,
  ): Promise<SubscriptionId> {
    return this.wsClient.subscribeEvent(filter, onMessage);
  }

  async unsubscribeEvent(id: SubscriptionId): Promise<boolean> {
    return this.wsClient.unsubscribeEvent(id);
  }

  async devInspectTransaction(
    sender: HaneulAddress,
    tx: Transaction | UnserializedSignableTransaction | string | Uint8Array,
    gasPrice: number | null = null,
    epoch: number | null = null,
  ): Promise<DevInspectResults> {
    try {
      let devInspectTxBytes;
      if (Transaction.is(tx)) {
        devInspectTxBytes = await tx.build({ provider: this });
      } else if (typeof tx === 'string') {
        devInspectTxBytes = tx;
      } else if (tx instanceof Uint8Array) {
        devInspectTxBytes = toB64(tx);
      } else {
        devInspectTxBytes = toB64(
          await new LocalTxnDataSerializer(this).serializeToBytesWithoutGasInfo(
            sender,
            tx,
          ),
        );
      }

      const resp = await this.client.requestWithType(
        'haneul_devInspectTransaction',
        [sender, devInspectTxBytes, gasPrice, epoch],
        DevInspectResults,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error dev inspect transaction with request type: ${err}`,
      );
    }
  }

  async dryRunTransaction(
    txBytes: Uint8Array,
  ): Promise<DryRunTransactionResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_dryRunTransaction',
        [toB64(txBytes)],
        DryRunTransactionResponse,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error dry running transaction with request type: ${err}`,
      );
    }
  }

  // Dynamic Fields
  async getDynamicFields(
    parent_object_id: ObjectId,
    cursor: ObjectId | null = null,
    limit: number | null = null,
  ): Promise<DynamicFieldPage> {
    try {
      if (
        !parent_object_id ||
        !isValidHaneulObjectId(normalizeHaneulObjectId(parent_object_id))
      ) {
        throw new Error('Invalid Haneul Object id');
      }
      const resp = await this.client.requestWithType(
        'haneul_getDynamicFields',
        [parent_object_id, cursor, limit],
        DynamicFieldPage,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error getting dynamic fields with request type: ${err} for parent_object_id: ${parent_object_id}, cursor: ${cursor} and limit: ${limit}.`,
      );
    }
  }

  async getDynamicFieldObject(
    parent_object_id: ObjectId,
    name: string | DynamicFieldName,
  ): Promise<HaneulObjectResponse> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getDynamicFieldObject',
        [parent_object_id, name],
        HaneulObjectResponse,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error getting dynamic field object with request type: ${err} for parent_object_id: ${parent_object_id} and name: ${name}.`,
      );
    }
  }

  // Checkpoints
  async getLatestCheckpointSequenceNumber(): Promise<number> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getLatestCheckpointSequenceNumber',
        [],
        number(),
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error fetching latest checkpoint sequence number: ${err}`,
      );
    }
  }

  async getCheckpoint(id: CheckpointDigest | number): Promise<Checkpoint> {
    try {
      const resp = await this.client.requestWithType(
        'haneul_getCheckpoint',
        [id],
        Checkpoint,
        this.options.skipDataValidation,
      );
      return resp;
    } catch (err) {
      throw new Error(
        `Error getting checkpoint with request type: ${err} for id: ${id}.`,
      );
    }
  }

  async getCommitteeInfo(epoch?: number): Promise<CommitteeInfo> {
    try {
      const committeeInfo = await this.client.requestWithType(
        'haneul_getCommitteeInfo',
        [epoch],
        CommitteeInfo,
      );

      return committeeInfo;
    } catch (error) {
      throw new Error(`Error getCommitteeInfo : ${error}`);
    }
  }
}
