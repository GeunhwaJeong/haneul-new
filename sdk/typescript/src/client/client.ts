// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import { fromB58, toB64, toHEX } from '@haneullabs/bcs';

import type { TransactionBlock } from '../builder/index.js';
import { isTransactionBlock } from '../builder/index.js';
import type { Keypair } from '../cryptography/index.js';
import {
	isValidHaneulAddress,
	isValidHaneulObjectId,
	isValidTransactionDigest,
	normalizeHaneulAddress,
	normalizeHaneulObjectId,
} from '../utils/haneul-types.js';
import { HaneulHTTPTransport } from './http-transport.js';
import type { HaneulTransport } from './http-transport.js';
import type {
	AddressMetrics,
	AllEpochsAddressMetrics,
	Checkpoint,
	CheckpointPage,
	CoinBalance,
	CoinMetadata,
	CoinSupply,
	CommitteeInfo,
	DelegatedStake,
	DevInspectResults,
	DevInspectTransactionBlockParams,
	DryRunTransactionBlockParams,
	DryRunTransactionBlockResponse,
	DynamicFieldPage,
	EpochInfo,
	EpochPage,
	ExecuteTransactionBlockParams,
	GetAllBalancesParams,
	GetAllCoinsParams,
	GetBalanceParams,
	GetCheckpointParams,
	GetCheckpointsParams,
	GetCoinMetadataParams,
	GetCoinsParams,
	GetCommitteeInfoParams,
	GetDynamicFieldObjectParams,
	GetDynamicFieldsParams,
	GetMoveFunctionArgTypesParams,
	GetNormalizedMoveFunctionParams,
	GetNormalizedMoveModuleParams,
	GetNormalizedMoveModulesByPackageParams,
	GetNormalizedMoveStructParams,
	GetObjectParams,
	GetOwnedObjectsParams,
	GetProtocolConfigParams,
	GetStakesByIdsParams,
	GetStakesParams,
	GetTotalSupplyParams,
	GetTransactionBlockParams,
	MoveCallMetrics,
	MultiGetObjectsParams,
	MultiGetTransactionBlocksParams,
	NetworkMetrics,
	ObjectRead,
	Order,
	PaginatedCoins,
	PaginatedEvents,
	PaginatedObjectsResponse,
	PaginatedTransactionResponse,
	ProtocolConfig,
	QueryEventsParams,
	QueryTransactionBlocksParams,
	ResolvedNameServiceNames,
	ResolveNameServiceAddressParams,
	ResolveNameServiceNamesParams,
	SubscribeEventParams,
	SubscribeTransactionParams,
	HaneulEvent,
	HaneulMoveFunctionArgType,
	HaneulMoveNormalizedFunction,
	HaneulMoveNormalizedModule,
	HaneulMoveNormalizedModules,
	HaneulMoveNormalizedStruct,
	HaneulObjectResponse,
	HaneulObjectResponseQuery,
	HaneulSystemStateSummary,
	HaneulTransactionBlockResponse,
	HaneulTransactionBlockResponseQuery,
	TransactionEffects,
	TryGetPastObjectParams,
	Unsubscribe,
	ValidatorsApy,
} from './types/index.js';

export interface PaginationArguments<Cursor> {
	/** Optional paging cursor */
	cursor?: Cursor;
	/** Maximum item returned per page */
	limit?: number | null;
}

export interface OrderArguments {
	order?: Order | null;
}

/**
 * Configuration options for the HaneulClient
 * You must provide either a `url` or a `transport`
 */
export type HaneulClientOptions = NetworkOrTransport;

export type NetworkOrTransport =
	| {
			url: string;
			transport?: never;
	  }
	| {
			transport: HaneulTransport;
			url?: never;
	  };

export const HANEUL_CLIENT_BRAND = Symbol.for('@haneullabs/HaneulClient');

export function isHaneulClient(client: unknown): client is HaneulClient {
	return (
		typeof client === 'object' &&
		client !== null &&
		(client as { [HANEUL_CLIENT_BRAND]: unknown })[HANEUL_CLIENT_BRAND] === true
	);
}

export class HaneulClient {
	protected transport: HaneulTransport;

	get [HANEUL_CLIENT_BRAND]() {
		return true;
	}

	/**
	 * Establish a connection to a Haneul RPC endpoint
	 *
	 * @param options configuration options for the API Client
	 */
	constructor(options: HaneulClientOptions) {
		this.transport = options.transport ?? new HaneulHTTPTransport({ url: options.url });
	}

	async getRpcApiVersion(): Promise<string | undefined> {
		const resp = await this.transport.request<{ info: { version: string } }>({
			method: 'rpc.discover',
			params: [],
		});

		return resp.info.version;
	}

	/**
	 * Get all Coin<`coin_type`> objects owned by an address.
	 */
	async getCoins(input: GetCoinsParams): Promise<PaginatedCoins> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}

		return await this.transport.request({
			method: 'haneulx_getCoins',
			params: [input.owner, input.coinType, input.cursor, input.limit],
		});
	}

	/**
	 * Get all Coin objects owned by an address.
	 */
	async getAllCoins(input: GetAllCoinsParams): Promise<PaginatedCoins> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}

		return await this.transport.request({
			method: 'haneulx_getAllCoins',
			params: [input.owner, input.cursor, input.limit],
		});
	}

	/**
	 * Get the total coin balance for one coin type, owned by the address owner.
	 */
	async getBalance(input: GetBalanceParams): Promise<CoinBalance> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}
		return await this.transport.request({
			method: 'haneulx_getBalance',
			params: [input.owner, input.coinType],
		});
	}

	/**
	 * Get the total coin balance for all coin types, owned by the address owner.
	 */
	async getAllBalances(input: GetAllBalancesParams): Promise<CoinBalance[]> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}
		return await this.transport.request({ method: 'haneulx_getAllBalances', params: [input.owner] });
	}

	/**
	 * Fetch CoinMetadata for a given coin type
	 */
	async getCoinMetadata(input: GetCoinMetadataParams): Promise<CoinMetadata | null> {
		return await this.transport.request({
			method: 'haneulx_getCoinMetadata',
			params: [input.coinType],
		});
	}

	/**
	 *  Fetch total supply for a coin
	 */
	async getTotalSupply(input: GetTotalSupplyParams): Promise<CoinSupply> {
		return await this.transport.request({
			method: 'haneulx_getTotalSupply',
			params: [input.coinType],
		});
	}

	/**
	 * Invoke any RPC method
	 * @param method the method to be invoked
	 * @param args the arguments to be passed to the RPC request
	 */
	async call<T = unknown>(method: string, params: unknown[]): Promise<T> {
		return await this.transport.request({ method, params });
	}

	/**
	 * Get Move function argument types like read, write and full access
	 */
	async getMoveFunctionArgTypes(
		input: GetMoveFunctionArgTypesParams,
	): Promise<HaneulMoveFunctionArgType[]> {
		return await this.transport.request({
			method: 'haneul_getMoveFunctionArgTypes',
			params: [input.package, input.module, input.function],
		});
	}

	/**
	 * Get a map from module name to
	 * structured representations of Move modules
	 */
	async getNormalizedMoveModulesByPackage(
		input: GetNormalizedMoveModulesByPackageParams,
	): Promise<HaneulMoveNormalizedModules> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveModulesByPackage',
			params: [input.package],
		});
	}

	/**
	 * Get a structured representation of Move module
	 */
	async getNormalizedMoveModule(
		input: GetNormalizedMoveModuleParams,
	): Promise<HaneulMoveNormalizedModule> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveModule',
			params: [input.package, input.module],
		});
	}

	/**
	 * Get a structured representation of Move function
	 */
	async getNormalizedMoveFunction(
		input: GetNormalizedMoveFunctionParams,
	): Promise<HaneulMoveNormalizedFunction> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveFunction',
			params: [input.package, input.module, input.function],
		});
	}

	/**
	 * Get a structured representation of Move struct
	 */
	async getNormalizedMoveStruct(
		input: GetNormalizedMoveStructParams,
	): Promise<HaneulMoveNormalizedStruct> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveStruct',
			params: [input.package, input.module, input.struct],
		});
	}

	/**
	 * Get all objects owned by an address
	 */
	async getOwnedObjects(input: GetOwnedObjectsParams): Promise<PaginatedObjectsResponse> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}

		return await this.transport.request({
			method: 'haneulx_getOwnedObjects',
			params: [
				input.owner,
				{
					filter: input.filter,
					options: input.options,
				} as HaneulObjectResponseQuery,
				input.cursor,
				input.limit,
			],
		});
	}

	/**
	 * Get details about an object
	 */
	async getObject(input: GetObjectParams): Promise<HaneulObjectResponse> {
		if (!input.id || !isValidHaneulObjectId(normalizeHaneulObjectId(input.id))) {
			throw new Error('Invalid Haneul Object id');
		}
		return await this.transport.request({
			method: 'haneul_getObject',
			params: [input.id, input.options],
		});
	}

	async tryGetPastObject(input: TryGetPastObjectParams): Promise<ObjectRead> {
		return await this.transport.request({
			method: 'haneul_tryGetPastObject',
			params: [input.id, input.version, input.options],
		});
	}

	/**
	 * Batch get details about a list of objects. If any of the object ids are duplicates the call will fail
	 */
	async multiGetObjects(input: MultiGetObjectsParams): Promise<HaneulObjectResponse[]> {
		input.ids.forEach((id) => {
			if (!id || !isValidHaneulObjectId(normalizeHaneulObjectId(id))) {
				throw new Error(`Invalid Haneul Object id ${id}`);
			}
		});
		const hasDuplicates = input.ids.length !== new Set(input.ids).size;
		if (hasDuplicates) {
			throw new Error(`Duplicate object ids in batch call ${input.ids}`);
		}

		return await this.transport.request({
			method: 'haneul_multiGetObjects',
			params: [input.ids, input.options],
		});
	}

	/**
	 * Get transaction blocks for a given query criteria
	 */
	async queryTransactionBlocks(
		input: QueryTransactionBlocksParams,
	): Promise<PaginatedTransactionResponse> {
		return await this.transport.request({
			method: 'haneulx_queryTransactionBlocks',
			params: [
				{
					filter: input.filter,
					options: input.options,
				} as HaneulTransactionBlockResponseQuery,
				input.cursor,
				input.limit,
				(input.order || 'descending') === 'descending',
			],
		});
	}

	async getTransactionBlock(
		input: GetTransactionBlockParams,
	): Promise<HaneulTransactionBlockResponse> {
		if (!isValidTransactionDigest(input.digest)) {
			throw new Error('Invalid Transaction digest');
		}
		return await this.transport.request({
			method: 'haneul_getTransactionBlock',
			params: [input.digest, input.options],
		});
	}

	async multiGetTransactionBlocks(
		input: MultiGetTransactionBlocksParams,
	): Promise<HaneulTransactionBlockResponse[]> {
		input.digests.forEach((d) => {
			if (!isValidTransactionDigest(d)) {
				throw new Error(`Invalid Transaction digest ${d}`);
			}
		});

		const hasDuplicates = input.digests.length !== new Set(input.digests).size;
		if (hasDuplicates) {
			throw new Error(`Duplicate digests in batch call ${input.digests}`);
		}

		return await this.transport.request({
			method: 'haneul_multiGetTransactionBlocks',
			params: [input.digests, input.options],
		});
	}

	async executeTransactionBlock(
		input: ExecuteTransactionBlockParams,
	): Promise<HaneulTransactionBlockResponse> {
		return await this.transport.request({
			method: 'haneul_executeTransactionBlock',
			params: [
				typeof input.transactionBlock === 'string'
					? input.transactionBlock
					: toB64(input.transactionBlock),
				Array.isArray(input.signature) ? input.signature : [input.signature],
				input.options,
				input.requestType,
			],
		});
	}

	async signAndExecuteTransactionBlock({
		transactionBlock,
		signer,
		...input
	}: {
		transactionBlock: Uint8Array | TransactionBlock;
		signer: Keypair;
	} & Omit<
		ExecuteTransactionBlockParams,
		'transactionBlock' | 'signature'
	>): Promise<HaneulTransactionBlockResponse> {
		let transactionBytes;

		if (transactionBlock instanceof Uint8Array) {
			transactionBytes = transactionBlock;
		} else {
			transactionBlock.setSenderIfNotSet(await signer.getPublicKey().toHaneulAddress());
			transactionBytes = await transactionBlock.build({ client: this });
		}

		const { signature, bytes } = await signer.signTransactionBlock(transactionBytes);

		return this.executeTransactionBlock({
			transactionBlock: bytes,
			signature,
			...input,
		});
	}

	/**
	 * Get total number of transactions
	 */

	async getTotalTransactionBlocks(): Promise<bigint> {
		const resp = await this.transport.request<string>({
			method: 'haneul_getTotalTransactionBlocks',
			params: [],
		});
		return BigInt(resp);
	}

	/**
	 * Getting the reference gas price for the network
	 */
	async getReferenceGasPrice(): Promise<bigint> {
		const resp = await this.transport.request<string>({
			method: 'haneulx_getReferenceGasPrice',
			params: [],
		});
		return BigInt(resp);
	}

	/**
	 * Return the delegated stakes for an address
	 */
	async getStakes(input: GetStakesParams): Promise<DelegatedStake[]> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}
		return await this.transport.request({ method: 'haneulx_getStakes', params: [input.owner] });
	}

	/**
	 * Return the delegated stakes queried by id.
	 */
	async getStakesByIds(input: GetStakesByIdsParams): Promise<DelegatedStake[]> {
		input.stakedHaneulIds.forEach((id) => {
			if (!id || !isValidHaneulObjectId(normalizeHaneulObjectId(id))) {
				throw new Error(`Invalid Haneul Stake id ${id}`);
			}
		});
		return await this.transport.request({
			method: 'haneulx_getStakesByIds',
			params: [input.stakedHaneulIds],
		});
	}

	/**
	 * Return the latest system state content.
	 */
	async getLatestHaneulSystemState(): Promise<HaneulSystemStateSummary> {
		return await this.transport.request({ method: 'haneulx_getLatestHaneulSystemState', params: [] });
	}

	/**
	 * Get events for a given query criteria
	 */
	async queryEvents(input: QueryEventsParams): Promise<PaginatedEvents> {
		return await this.transport.request({
			method: 'haneulx_queryEvents',
			params: [
				input.query,
				input.cursor,
				input.limit,
				(input.order || 'descending') === 'descending',
			],
		});
	}

	/**
	 * Subscribe to get notifications whenever an event matching the filter occurs
	 */
	async subscribeEvent(
		input: SubscribeEventParams & {
			/** function to run when we receive a notification of a new event matching the filter */
			onMessage: (event: HaneulEvent) => void;
		},
	): Promise<Unsubscribe> {
		return this.transport.subscribe({
			method: 'haneulx_subscribeEvent',
			unsubscribe: 'haneulx_unsubscribeEvent',
			params: [input.filter],
			onMessage: input.onMessage,
		});
	}

	async subscribeTransaction(
		input: SubscribeTransactionParams & {
			/** function to run when we receive a notification of a new event matching the filter */
			onMessage: (event: TransactionEffects) => void;
		},
	): Promise<Unsubscribe> {
		return this.transport.subscribe({
			method: 'haneulx_subscribeTransaction',
			unsubscribe: 'haneulx_unsubscribeTransaction',
			params: [input.filter],
			onMessage: input.onMessage,
		});
	}

	/**
	 * Runs the transaction block in dev-inspect mode. Which allows for nearly any
	 * transaction (or Move call) with any arguments. Detailed results are
	 * provided, including both the transaction effects and any return values.
	 */
	async devInspectTransactionBlock(
		input: DevInspectTransactionBlockParams,
	): Promise<DevInspectResults> {
		let devInspectTxBytes;
		if (isTransactionBlock(input.transactionBlock)) {
			input.transactionBlock.setSenderIfNotSet(input.sender);
			devInspectTxBytes = toB64(
				await input.transactionBlock.build({
					client: this,
					onlyTransactionKind: true,
				}),
			);
		} else if (typeof input.transactionBlock === 'string') {
			devInspectTxBytes = input.transactionBlock;
		} else if (input.transactionBlock instanceof Uint8Array) {
			devInspectTxBytes = toB64(input.transactionBlock);
		} else {
			throw new Error('Unknown transaction block format.');
		}

		return await this.transport.request({
			method: 'haneul_devInspectTransactionBlock',
			params: [input.sender, devInspectTxBytes, input.gasPrice, input.epoch],
		});
	}

	/**
	 * Dry run a transaction block and return the result.
	 */
	async dryRunTransactionBlock(
		input: DryRunTransactionBlockParams,
	): Promise<DryRunTransactionBlockResponse> {
		return await this.transport.request({
			method: 'haneul_dryRunTransactionBlock',
			params: [
				typeof input.transactionBlock === 'string'
					? input.transactionBlock
					: toB64(input.transactionBlock),
			],
		});
	}

	/**
	 * Return the list of dynamic field objects owned by an object
	 */
	async getDynamicFields(input: GetDynamicFieldsParams): Promise<DynamicFieldPage> {
		if (!input.parentId || !isValidHaneulObjectId(normalizeHaneulObjectId(input.parentId))) {
			throw new Error('Invalid Haneul Object id');
		}
		return await this.transport.request({
			method: 'haneulx_getDynamicFields',
			params: [input.parentId, input.cursor, input.limit],
		});
	}

	/**
	 * Return the dynamic field object information for a specified object
	 */
	async getDynamicFieldObject(input: GetDynamicFieldObjectParams): Promise<HaneulObjectResponse> {
		return await this.transport.request({
			method: 'haneulx_getDynamicFieldObject',
			params: [input.parentId, input.name],
		});
	}

	/**
	 * Get the sequence number of the latest checkpoint that has been executed
	 */
	async getLatestCheckpointSequenceNumber(): Promise<string> {
		const resp = await this.transport.request({
			method: 'haneul_getLatestCheckpointSequenceNumber',
			params: [],
		});
		return String(resp);
	}

	/**
	 * Returns information about a given checkpoint
	 */
	async getCheckpoint(input: GetCheckpointParams): Promise<Checkpoint> {
		return await this.transport.request({ method: 'haneul_getCheckpoint', params: [input.id] });
	}

	/**
	 * Returns historical checkpoints paginated
	 */
	async getCheckpoints(
		input: PaginationArguments<CheckpointPage['nextCursor']> & GetCheckpointsParams,
	): Promise<CheckpointPage> {
		return await this.transport.request({
			method: 'haneul_getCheckpoints',
			params: [input.cursor, input?.limit, input.descendingOrder],
		});
	}

	/**
	 * Return the committee information for the asked epoch
	 */
	async getCommitteeInfo(input?: GetCommitteeInfoParams): Promise<CommitteeInfo> {
		return await this.transport.request({
			method: 'haneulx_getCommitteeInfo',
			params: [input?.epoch],
		});
	}

	async getNetworkMetrics(): Promise<NetworkMetrics> {
		return await this.transport.request({ method: 'haneulx_getNetworkMetrics', params: [] });
	}

	async getAddressMetrics(): Promise<AddressMetrics> {
		return await this.transport.request({ method: 'haneulx_getLatestAddressMetrics', params: [] });
	}

	async getAllEpochAddressMetrics(input?: {
		descendingOrder?: boolean;
	}): Promise<AllEpochsAddressMetrics> {
		return await this.transport.request({
			method: 'haneulx_getAllEpochAddressMetrics',
			params: [input?.descendingOrder],
		});
	}

	/**
	 * Return the committee information for the asked epoch
	 */
	async getEpochs(
		input?: {
			descendingOrder?: boolean;
		} & PaginationArguments<EpochPage['nextCursor']>,
	): Promise<EpochPage> {
		return await this.transport.request({
			method: 'haneulx_getEpochs',
			params: [input?.cursor, input?.limit, input?.descendingOrder],
		});
	}

	/**
	 * Returns list of top move calls by usage
	 */
	async getMoveCallMetrics(): Promise<MoveCallMetrics> {
		return await this.transport.request({ method: 'haneulx_getMoveCallMetrics', params: [] });
	}

	/**
	 * Return the committee information for the asked epoch
	 */
	async getCurrentEpoch(): Promise<EpochInfo> {
		return await this.transport.request({ method: 'haneulx_getCurrentEpoch', params: [] });
	}

	/**
	 * Return the Validators APYs
	 */
	async getValidatorsApy(): Promise<ValidatorsApy> {
		return await this.transport.request({ method: 'haneulx_getValidatorsApy', params: [] });
	}

	// TODO: Migrate this to `haneul_getChainIdentifier` once it is widely available.
	async getChainIdentifier(): Promise<string> {
		const checkpoint = await this.getCheckpoint({ id: '0' });
		const bytes = fromB58(checkpoint.digest);
		return toHEX(bytes.slice(0, 4));
	}

	async resolveNameServiceAddress(input: ResolveNameServiceAddressParams): Promise<string | null> {
		return await this.transport.request({
			method: 'haneulx_resolveNameServiceAddress',
			params: [input.name],
		});
	}

	async resolveNameServiceNames(
		input: ResolveNameServiceNamesParams,
	): Promise<ResolvedNameServiceNames> {
		return await this.transport.request({
			method: 'haneulx_resolveNameServiceNames',
			params: [input.address, input.cursor, input.limit],
		});
	}

	async getProtocolConfig(input?: GetProtocolConfigParams): Promise<ProtocolConfig> {
		return await this.transport.request({
			method: 'haneul_getProtocolConfig',
			params: [input?.version],
		});
	}

	/**
	 * Wait for a transaction block result to be available over the API.
	 * This can be used in conjunction with `executeTransactionBlock` to wait for the transaction to
	 * be available via the API.
	 * This currently polls the `getTransactionBlock` API to check for the transaction.
	 */
	async waitForTransactionBlock({
		signal,
		timeout = 60 * 1000,
		pollInterval = 2 * 1000,
		...input
	}: {
		/** An optional abort signal that can be used to cancel */
		signal?: AbortSignal;
		/** The amount of time to wait for a transaction block. Defaults to one minute. */
		timeout?: number;
		/** The amount of time to wait between checks for the transaction block. Defaults to 2 seconds. */
		pollInterval?: number;
	} & Parameters<HaneulClient['getTransactionBlock']>[0]): Promise<HaneulTransactionBlockResponse> {
		const timeoutSignal = AbortSignal.timeout(timeout);
		const timeoutPromise = new Promise((_, reject) => {
			timeoutSignal.addEventListener('abort', () => reject(timeoutSignal.reason));
		});

		timeoutPromise.catch(() => {
			// Swallow unhandled rejections that might be thrown after early return
		});

		while (!timeoutSignal.aborted) {
			signal?.throwIfAborted();
			try {
				return await this.getTransactionBlock(input);
			} catch (e) {
				// Wait for either the next poll interval, or the timeout.
				await Promise.race([
					new Promise((resolve) => setTimeout(resolve, pollInterval)),
					timeoutPromise,
				]);
			}
		}

		timeoutSignal.throwIfAborted();

		// This should never happen, because the above case should always throw, but just adding it in the event that something goes horribly wrong.
		throw new Error('Unexpected error while waiting for transaction block.');
	}
}
