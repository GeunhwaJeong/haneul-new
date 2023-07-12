// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0
import type {
	ExecuteTransactionRequestType,
	ObjectId,
	HaneulEventFilter,
	TransactionDigest,
	HaneulTransactionBlockResponseQuery,
	Order,
	CoinMetadata,
	CheckpointDigest,
	HaneulObjectDataOptions,
	HaneulTransactionBlockResponseOptions,
	HaneulEvent,
	HaneulObjectResponseQuery,
	TransactionFilter,
	TransactionEffects,
	Unsubscribe,
	PaginatedTransactionResponse,
	HaneulAddress,
	HaneulMoveFunctionArgTypes,
	HaneulMoveNormalizedFunction,
	HaneulMoveNormalizedModule,
	HaneulMoveNormalizedModules,
	HaneulMoveNormalizedStruct,
	HaneulTransactionBlockResponse,
	PaginatedEvents,
	DevInspectResults,
	PaginatedCoins,
	HaneulObjectResponse,
	DelegatedStake,
	CoinBalance,
	CoinSupply,
	Checkpoint,
	CommitteeInfo,
	DryRunTransactionBlockResponse,
	HaneulSystemStateSummary,
	PaginatedObjectsResponse,
	ValidatorsApy,
	MoveCallMetrics,
	ObjectRead,
	ResolvedNameServiceNames,
	ProtocolConfig,
	EpochInfo,
	EpochPage,
	CheckpointPage,
	DynamicFieldName,
	DynamicFieldPage,
	NetworkMetrics,
	AddressMetrics,
} from '../types/index.js';
import {
	isValidTransactionDigest,
	isValidHaneulAddress,
	isValidHaneulObjectId,
	normalizeHaneulAddress,
	normalizeHaneulObjectId,
} from '../utils/haneul-types.js';
import { fromB58, toB64, toHEX } from '@haneullabs/bcs';
import type { SerializedSignature } from '../cryptography/signature.js';
import { TransactionBlock } from '../builder/index.js';
import { HaneulHTTPTransport } from './http-transport.js';
import type { HaneulTransport } from './http-transport.js';

export * from './http-transport.js';
export * from './network.js';

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

export class HaneulClient {
	protected transport: HaneulTransport;
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
	async getCoins(
		input: {
			owner: HaneulAddress;
			coinType?: string | null;
		} & PaginationArguments<PaginatedCoins['nextCursor']>,
	): Promise<PaginatedCoins> {
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
	async getAllCoins(
		input: {
			owner: HaneulAddress;
		} & PaginationArguments<PaginatedCoins['nextCursor']>,
	): Promise<PaginatedCoins> {
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
	async getBalance(input: {
		owner: HaneulAddress;
		/** optional fully qualified type names for the coin (e.g., 0x168da5bf1f48dafc111b0a488fa454aca95e0b5e::usdc::USDC), default to 0x2::haneul::HANEUL if not specified. */
		coinType?: string | null;
	}): Promise<CoinBalance> {
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
	async getAllBalances(input: { owner: HaneulAddress }): Promise<CoinBalance[]> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}
		return await this.transport.request({ method: 'haneulx_getAllBalances', params: [input.owner] });
	}

	/**
	 * Fetch CoinMetadata for a given coin type
	 */
	async getCoinMetadata(input: { coinType: string }): Promise<CoinMetadata | null> {
		return await this.transport.request({
			method: 'haneulx_getCoinMetadata',
			params: [input.coinType],
		});
	}

	/**
	 *  Fetch total supply for a coin
	 */
	async getTotalSupply(input: { coinType: string }): Promise<CoinSupply> {
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
	async call(method: string, params: unknown[]): Promise<unknown> {
		return await this.transport.request({ method, params });
	}

	/**
	 * Get Move function argument types like read, write and full access
	 */
	async getMoveFunctionArgTypes(input: {
		package: string;
		module: string;
		function: string;
	}): Promise<HaneulMoveFunctionArgTypes> {
		return await this.transport.request({
			method: 'haneul_getMoveFunctionArgTypes',
			params: [input.package, input.module, input.function],
		});
	}

	/**
	 * Get a map from module name to
	 * structured representations of Move modules
	 */
	async getNormalizedMoveModulesByPackage(input: {
		package: string;
	}): Promise<HaneulMoveNormalizedModules> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveModulesByPackage',
			params: [input.package],
		});
	}

	/**
	 * Get a structured representation of Move module
	 */
	async getNormalizedMoveModule(input: {
		package: string;
		module: string;
	}): Promise<HaneulMoveNormalizedModule> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveModule',
			params: [input.package, input.module],
		});
	}

	/**
	 * Get a structured representation of Move function
	 */
	async getNormalizedMoveFunction(input: {
		package: string;
		module: string;
		function: string;
	}): Promise<HaneulMoveNormalizedFunction> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveFunction',
			params: [input.package, input.module, input.function],
		});
	}

	/**
	 * Get a structured representation of Move struct
	 */
	async getNormalizedMoveStruct(input: {
		package: string;
		module: string;
		struct: string;
	}): Promise<HaneulMoveNormalizedStruct> {
		return await this.transport.request({
			method: 'haneul_getNormalizedMoveStruct',
			params: [input.package, input.module, input.struct],
		});
	}

	/**
	 * Get all objects owned by an address
	 */
	async getOwnedObjects(
		input: {
			owner: HaneulAddress;
		} & PaginationArguments<PaginatedObjectsResponse['nextCursor']> &
			HaneulObjectResponseQuery,
	): Promise<PaginatedObjectsResponse> {
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
	async getObject(input: {
		id: ObjectId;
		options?: HaneulObjectDataOptions;
	}): Promise<HaneulObjectResponse> {
		if (!input.id || !isValidHaneulObjectId(normalizeHaneulObjectId(input.id))) {
			throw new Error('Invalid Haneul Object id');
		}
		return await this.transport.request({
			method: 'haneul_getObject',
			params: [input.id, input.options],
		});
	}

	async tryGetPastObject(input: {
		id: ObjectId;
		version: number;
		options?: HaneulObjectDataOptions;
	}): Promise<ObjectRead> {
		return await this.transport.request({
			method: 'haneul_tryGetPastObject',
			params: [input.id, input.version, input.options],
		});
	}

	/**
	 * Batch get details about a list of objects. If any of the object ids are duplicates the call will fail
	 */
	async multiGetObjects(input: {
		ids: ObjectId[];
		options?: HaneulObjectDataOptions;
	}): Promise<HaneulObjectResponse[]> {
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
		input: HaneulTransactionBlockResponseQuery &
			PaginationArguments<PaginatedTransactionResponse['nextCursor']> &
			OrderArguments,
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

	async getTransactionBlock(input: {
		digest: TransactionDigest;
		options?: HaneulTransactionBlockResponseOptions;
	}): Promise<HaneulTransactionBlockResponse> {
		if (!isValidTransactionDigest(input.digest)) {
			throw new Error('Invalid Transaction digest');
		}
		return await this.transport.request({
			method: 'haneul_getTransactionBlock',
			params: [input.digest, input.options],
		});
	}

	async multiGetTransactionBlocks(input: {
		digests: TransactionDigest[];
		options?: HaneulTransactionBlockResponseOptions;
	}): Promise<HaneulTransactionBlockResponse[]> {
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

	async executeTransactionBlock(input: {
		transactionBlock: Uint8Array | string;
		signature: SerializedSignature | SerializedSignature[];
		options?: HaneulTransactionBlockResponseOptions;
		requestType?: ExecuteTransactionRequestType;
	}): Promise<HaneulTransactionBlockResponse> {
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
	async getStakes(input: { owner: HaneulAddress }): Promise<DelegatedStake[]> {
		if (!input.owner || !isValidHaneulAddress(normalizeHaneulAddress(input.owner))) {
			throw new Error('Invalid Haneul address');
		}
		return await this.transport.request({ method: 'haneulx_getStakes', params: [input.owner] });
	}

	/**
	 * Return the delegated stakes queried by id.
	 */
	async getStakesByIds(input: { stakedHaneulIds: ObjectId[] }): Promise<DelegatedStake[]> {
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
	async queryEvents(
		input: {
			/** the event query criteria. */
			query: HaneulEventFilter;
		} & PaginationArguments<PaginatedEvents['nextCursor']> &
			OrderArguments,
	): Promise<PaginatedEvents> {
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
	async subscribeEvent(input: {
		/** filter describing the subset of events to follow */
		filter: HaneulEventFilter;
		/** function to run when we receive a notification of a new event matching the filter */
		onMessage: (event: HaneulEvent) => void;
	}): Promise<Unsubscribe> {
		return this.transport.subscribe({
			method: 'haneulx_subscribeEvent',
			unsubscribe: 'haneulx_unsubscribeEvent',
			params: [input.filter],
			onMessage: input.onMessage,
		});
	}

	async subscribeTransaction(input: {
		/** filter describing the subset of events to follow */
		filter: TransactionFilter;
		/** function to run when we receive a notification of a new event matching the filter */
		onMessage: (event: TransactionEffects) => void;
	}): Promise<Unsubscribe> {
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
	async devInspectTransactionBlock(input: {
		transactionBlock: TransactionBlock | string | Uint8Array;
		sender: HaneulAddress;
		/** Default to use the network reference gas price stored in the Haneul System State object */
		gasPrice?: bigint | number | null;
		/** optional. Default to use the current epoch number stored in the Haneul System State object */
		epoch?: string | null;
	}): Promise<DevInspectResults> {
		let devInspectTxBytes;
		if (TransactionBlock.is(input.transactionBlock)) {
			input.transactionBlock.setSenderIfNotSet(input.sender);
			devInspectTxBytes = toB64(
				await input.transactionBlock.build({
					provider: this,
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
	async dryRunTransactionBlock(input: {
		transactionBlock: Uint8Array | string;
	}): Promise<DryRunTransactionBlockResponse> {
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
	async getDynamicFields(
		input: {
			/** The id of the parent object */
			parentId: ObjectId;
		} & PaginationArguments<DynamicFieldPage['nextCursor']>,
	): Promise<DynamicFieldPage> {
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
	async getDynamicFieldObject(input: {
		/** The ID of the quered parent object */
		parentId: ObjectId;
		/** The name of the dynamic field */
		name: string | DynamicFieldName;
	}): Promise<HaneulObjectResponse> {
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
	async getCheckpoint(input: {
		/** The checkpoint digest or sequence number */
		id: CheckpointDigest | string;
	}): Promise<Checkpoint> {
		return await this.transport.request({ method: 'haneul_getCheckpoint', params: [input.id] });
	}

	/**
	 * Returns historical checkpoints paginated
	 */
	async getCheckpoints(
		input: {
			/** query result ordering, default to false (ascending order), oldest record first */
			descendingOrder: boolean;
		} & PaginationArguments<CheckpointPage['nextCursor']>,
	): Promise<CheckpointPage> {
		return await this.transport.request({
			method: 'haneul_getCheckpoints',
			params: [input.cursor, input?.limit, input.descendingOrder],
		});
	}

	/**
	 * Return the committee information for the asked epoch
	 */
	async getCommitteeInfo(input?: {
		/** The epoch of interest. If null, default to the latest epoch */
		epoch?: string | null;
	}): Promise<CommitteeInfo> {
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

	async resolveNameServiceAddress(input: { name: string }): Promise<HaneulAddress | null> {
		return await this.transport.request({
			method: 'haneulx_resolveNameServiceAddress',
			params: [input.name],
		});
	}

	async resolveNameServiceNames(
		input: {
			address: string;
		} & PaginationArguments<ResolvedNameServiceNames['nextCursor']>,
	): Promise<ResolvedNameServiceNames> {
		return await this.transport.request({
			method: 'haneulx_resolveNameServiceNames',
			params: [input.address],
		});
	}

	async getProtocolConfig(input?: { version?: string }): Promise<ProtocolConfig> {
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
