// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { getFullnodeUrl, HaneulClient } from '@haneullabs/haneul.js/client';
import type { CoinStruct } from '@haneullabs/haneul.js/client';
import { decodeHaneulPrivateKey } from '@haneullabs/haneul.js/cryptography';
import type { Keypair, Signer } from '@haneullabs/haneul.js/cryptography';
import { Ed25519Keypair } from '@haneullabs/haneul.js/keypairs/ed25519';
import type {
	TransactionObjectArgument,
	TransactionObjectInput,
} from '@haneullabs/haneul.js/transactions';
import { TransactionBlock } from '@haneullabs/haneul.js/transactions';
import { normalizeStructTag, normalizeHaneulAddress, HANEUL_TYPE_ARG, toB64 } from '@haneullabs/haneul.js/utils';

import type { ZkBagContractOptions } from './zk-bag.js';
import { MAINNET_CONTRACT_IDS, ZkBag } from './zk-bag.js';

interface ZkSendLinkRedirect {
	url: string;
	name?: string;
}

export interface ZkSendLinkBuilderOptions {
	host?: string;
	path?: string;
	keypair?: Keypair;
	network?: 'mainnet' | 'testnet';
	client?: HaneulClient;
	sender: string;
	redirect?: ZkSendLinkRedirect;
	contract?: ZkBagContractOptions | null;
}

const DEFAULT_ZK_SEND_LINK_OPTIONS = {
	host: 'https://zksend.com',
	path: '/claim',
	network: 'mainnet' as const,
};

const HANEUL_COIN_TYPE = normalizeStructTag(HANEUL_TYPE_ARG);

export interface CreateZkSendLinkOptions {
	transactionBlock?: TransactionBlock;
	calculateGas?: (options: {
		balances: Map<string, bigint>;
		objects: TransactionObjectInput[];
		gasEstimateFromDryRun: bigint;
	}) => Promise<bigint> | bigint;
}

export class ZkSendLinkBuilder {
	objectIds = new Set<string>();
	balances = new Map<string, bigint>();
	sender: string;
	#host: string;
	#path: string;
	keypair: Keypair;
	#client: HaneulClient;
	#redirect?: ZkSendLinkRedirect;
	#coinsByType = new Map<string, CoinStruct[]>();
	#contract?: ZkBag<ZkBagContractOptions>;

	constructor({
		host = DEFAULT_ZK_SEND_LINK_OPTIONS.host,
		path = DEFAULT_ZK_SEND_LINK_OPTIONS.path,
		keypair = new Ed25519Keypair(),
		network = DEFAULT_ZK_SEND_LINK_OPTIONS.network,
		client = new HaneulClient({ url: getFullnodeUrl(network) }),
		sender,
		redirect,
		contract = network === 'mainnet' ? MAINNET_CONTRACT_IDS : undefined,
	}: ZkSendLinkBuilderOptions) {
		this.#host = host;
		this.#path = path;
		this.#redirect = redirect;
		this.keypair = keypair;
		this.#client = client;
		this.sender = normalizeHaneulAddress(sender);

		if (contract) {
			this.#contract = new ZkBag(contract.packageId, contract);
		}
	}

	addClaimableMist(amount: bigint) {
		this.addClaimableBalance(HANEUL_COIN_TYPE, amount);
	}

	addClaimableBalance(coinType: string, amount: bigint) {
		const normalizedType = normalizeStructTag(coinType);
		this.balances.set(normalizedType, (this.balances.get(normalizedType) ?? 0n) + amount);
	}

	addClaimableObject(id: string) {
		this.objectIds.add(id);
	}

	getLink(): string {
		const link = new URL(this.#host);
		link.pathname = this.#path;
		link.hash = `${this.#contract ? '$' : ''}${toB64(
			decodeHaneulPrivateKey(this.keypair.getSecretKey()).secretKey,
		)}`;

		if (this.#redirect) {
			link.searchParams.set('redirect_url', this.#redirect.url);
			if (this.#redirect.name) {
				link.searchParams.set('name', this.#redirect.name);
			}
		}

		return link.toString();
	}

	async create({
		signer,
		...options
	}: CreateZkSendLinkOptions & {
		signer: Signer;
		waitForTransactionBlock?: boolean;
	}) {
		const txb = await this.createSendTransaction(options);

		const result = await this.#client.signAndExecuteTransactionBlock({
			transactionBlock: await txb.build({ client: this.#client }),
			signer,
		});

		if (options.waitForTransactionBlock) {
			await this.#client.waitForTransactionBlock({ digest: result.digest });
		}

		return result;
	}
	async createSendTransaction({
		transactionBlock = new TransactionBlock(),
		calculateGas,
	}: CreateZkSendLinkOptions = {}) {
		if (!this.#contract) {
			return this.#createSendTransactionWithoutContract({ transactionBlock, calculateGas });
		}

		transactionBlock.setSenderIfNotSet(this.sender);

		return ZkSendLinkBuilder.createLinks({
			transactionBlock,
			client: this.#client,
			contract: this.#contract.ids,
			links: [this],
		});
	}

	async createSendToAddressTransaction({
		transactionBlock = new TransactionBlock(),
		address,
	}: CreateZkSendLinkOptions & {
		address: string;
	}) {
		const objectsToTransfer = (await this.#objectsToTransfer(transactionBlock)).map(
			(obj) => obj.ref,
		);

		transactionBlock.setSenderIfNotSet(this.sender);
		transactionBlock.transferObjects(objectsToTransfer, address);

		return transactionBlock;
	}

	async #objectsToTransfer(txb: TransactionBlock) {
		const objectIDs = [...this.objectIds];
		const refsWithType: {
			ref: TransactionObjectArgument;
			type: string;
		}[] = (
			await this.#client.multiGetObjects({
				ids: objectIDs,
				options: {
					showType: true,
				},
			})
		).map((res, i) => {
			if (!res.data || res.error) {
				throw new Error(`Failed to load object ${objectIDs[i]} (${res.error?.code})`);
			}

			return {
				ref: txb.objectRef({
					version: res.data.version,
					digest: res.data.digest,
					objectId: res.data.objectId,
				}),
				type: res.data.type!,
			};
		});

		txb.setSenderIfNotSet(this.sender);

		for (const [coinType, amount] of this.balances) {
			if (coinType === HANEUL_COIN_TYPE) {
				const [haneul] = txb.splitCoins(txb.gas, [amount]);
				refsWithType.push({
					ref: haneul,
					type: `0x2::coin::Coin<${coinType}>`,
				} as never);
			} else {
				const coins = (await this.#getCoinsByType(coinType)).map((coin) => coin.coinObjectId);

				if (coins.length > 1) {
					txb.mergeCoins(coins[0], coins.slice(1));
				}
				const [split] = txb.splitCoins(coins[0], [amount]);
				refsWithType.push({
					ref: split,
					type: `0x2::coin::Coin<${coinType}>`,
				});
			}
		}

		return refsWithType;
	}

	async #createSendTransactionWithoutContract({
		transactionBlock: txb = new TransactionBlock(),
		calculateGas,
	}: CreateZkSendLinkOptions = {}) {
		const gasEstimateFromDryRun = await this.#estimateClaimGasFee();
		const baseGasAmount = calculateGas
			? await calculateGas({
					balances: this.balances,
					objects: [...this.objectIds],
					gasEstimateFromDryRun,
			  })
			: gasEstimateFromDryRun * 2n;

		// Ensure that rounded gas is not less than the calculated gas
		const gasWithBuffer = baseGasAmount + 1013n;
		// Ensure that gas amount ends in 987
		const roundedGasAmount = gasWithBuffer - (gasWithBuffer % 1000n) - 13n;

		const address = this.keypair.toHaneulAddress();
		const objectsToTransfer = (await this.#objectsToTransfer(txb)).map((obj) => obj.ref);
		const [gas] = txb.splitCoins(txb.gas, [roundedGasAmount]);
		objectsToTransfer.push(gas);

		txb.setSenderIfNotSet(this.sender);
		txb.transferObjects(objectsToTransfer, address);

		return txb;
	}

	async #estimateClaimGasFee(): Promise<bigint> {
		const txb = new TransactionBlock();
		txb.setSender(this.sender);
		txb.setGasPayment([]);
		txb.transferObjects([txb.gas], this.keypair.toHaneulAddress());

		const idsToTransfer = [...this.objectIds];

		for (const [coinType] of this.balances) {
			const coins = await this.#getCoinsByType(coinType);

			if (!coins.length) {
				throw new Error(`Sending account does not contain any coins of type ${coinType}`);
			}

			idsToTransfer.push(coins[0].coinObjectId);
		}

		if (idsToTransfer.length > 0) {
			txb.transferObjects(
				idsToTransfer.map((id) => txb.object(id)),
				this.keypair.toHaneulAddress(),
			);
		}

		const result = await this.#client.dryRunTransactionBlock({
			transactionBlock: await txb.build({ client: this.#client }),
		});

		return (
			BigInt(result.effects.gasUsed.computationCost) +
			BigInt(result.effects.gasUsed.storageCost) -
			BigInt(result.effects.gasUsed.storageRebate)
		);
	}

	async #getCoinsByType(coinType: string) {
		if (this.#coinsByType.has(coinType)) {
			return this.#coinsByType.get(coinType)!;
		}

		const coins = await this.#client.getCoins({
			coinType,
			owner: this.sender,
		});

		this.#coinsByType.set(coinType, coins.data);

		return coins.data;
	}

	static async createLinks({
		links,
		network = 'mainnet',
		client = new HaneulClient({ url: getFullnodeUrl(network) }),
		transactionBlock = new TransactionBlock(),
		contract: contractIds = MAINNET_CONTRACT_IDS,
	}: {
		transactionBlock?: TransactionBlock;
		client?: HaneulClient;
		network?: 'mainnet' | 'testnet';
		links: ZkSendLinkBuilder[];
		contract?: ZkBagContractOptions;
	}) {
		const contract = new ZkBag(contractIds.packageId, contractIds);
		const store = transactionBlock.object(contract.ids.bagStoreId);

		const coinsByType = new Map<string, CoinStruct[]>();
		const allIds = links.flatMap((link) => [...link.objectIds]);

		await Promise.all(
			[...new Set(links.flatMap((link) => [...link.balances.keys()]))].map(async (coinType) => {
				const coins = await client.getCoins({
					coinType,
					owner: links[0].sender,
				});

				coinsByType.set(
					coinType,
					coins.data.filter((coin) => !allIds.includes(coin.coinObjectId)),
				);
			}),
		);

		const objectRefs = new Map<
			string,
			{
				ref: TransactionObjectArgument;
				type: string;
			}
		>();

		const pageSize = 50;
		let offset = 0;
		while (offset < allIds.length) {
			let chunk = allIds.slice(offset, offset + pageSize);
			offset += pageSize;

			const objects = await client.multiGetObjects({
				ids: chunk,
				options: {
					showType: true,
				},
			});

			for (const [i, res] of objects.entries()) {
				if (!res.data || res.error) {
					throw new Error(`Failed to load object ${chunk[i]} (${res.error?.code})`);
				}
				objectRefs.set(chunk[i], {
					ref: transactionBlock.objectRef({
						version: res.data.version,
						digest: res.data.digest,
						objectId: res.data.objectId,
					}),
					type: res.data.type!,
				});
			}
		}

		const mergedCoins = new Map<string, TransactionObjectArgument>([
			[HANEUL_COIN_TYPE, transactionBlock.gas],
		]);

		for (const [coinType, coins] of coinsByType) {
			if (coinType === HANEUL_COIN_TYPE) {
				continue;
			}

			const [first, ...rest] = coins.map((coin) =>
				transactionBlock.objectRef({
					objectId: coin.coinObjectId,
					version: coin.version,
					digest: coin.digest,
				}),
			);
			if (rest.length > 0) {
				transactionBlock.mergeCoins(first, rest);
			}
			mergedCoins.set(coinType, transactionBlock.object(first));
		}

		for (const link of links) {
			const receiver = link.keypair.toHaneulAddress();
			contract.new(transactionBlock, { arguments: [store, receiver] });

			link.objectIds.forEach((id) => {
				const object = objectRefs.get(id);
				if (!object) {
					throw new Error(`Object ${id} not found`);
				}
				contract.add(transactionBlock, {
					arguments: [store, receiver, object.ref],
					typeArguments: [object.type],
				});
			});
		}

		for (const [coinType, merged] of mergedCoins) {
			const linksWithCoin = links.filter((link) => link.balances.has(coinType));
			if (linksWithCoin.length === 0) {
				continue;
			}

			const balances = linksWithCoin.map((link) => link.balances.get(coinType)!);
			const splits = transactionBlock.splitCoins(merged, balances);
			for (const [i, link] of linksWithCoin.entries()) {
				contract.add(transactionBlock, {
					arguments: [store, link.keypair.toHaneulAddress(), splits[i]],
					typeArguments: [`0x2::coin::Coin<${coinType}>`],
				});
			}
		}

		return transactionBlock;
	}
}
