// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const concepts = [
	'concepts',
	'concepts/components',
	{
		type: 'category',
		label: 'App Developers',
		link: {
			type: 'doc',
			id: 'concepts/app-devs',
		},
		items: [
			'concepts/graphql-rpc',
			{
				type: 'category',
				label: 'Object Model',
				link: {
					type: 'doc',
					id: 'concepts/object-model',
				},
				items: [
					{
						type: 'category',
						label: 'Object Ownership',
						link: {
							type: 'doc',
							id: 'concepts/object-ownership',
						},
						items: [
							'concepts/object-ownership/address-owned',
							'concepts/object-ownership/immutable',
							'concepts/object-ownership/shared',
							'concepts/object-ownership/wrapped',
						],
					},
					{
						type: 'category',
						label: 'Dynamic Fields',
						link: {
							type: 'doc',
							id: 'concepts/dynamic-fields',
						},
						items: ['concepts/dynamic-fields/tables-bags'],
					},
					{
						type: 'category',
						label: 'Transfers',
						link: {
							type: 'doc',
							id: 'concepts/transfers',
						},
						items: ['concepts/transfers/custom-rules', 'concepts/transfers/transfer-to-object'],
					},
					'concepts/events',
					'concepts/versioning',
				],
			},
			{
				type: 'category',
				label: 'Move Overview',
				link: {
					type: 'doc',
					id: 'concepts/haneul-move-concepts',
				},
				items: [
					'concepts/haneul-move-concepts/strings',
					'concepts/haneul-move-concepts/collections',
					'concepts/haneul-move-concepts/init',
					'concepts/haneul-move-concepts/entry-functions',
					'concepts/haneul-move-concepts/one-time-witness',
					{
						type: 'category',
						label: 'Package Upgrades',
						link: {
							type: 'doc',
							id: 'concepts/haneul-move-concepts/packages',
						},
						items: [
							'concepts/haneul-move-concepts/packages/upgrade',
							'concepts/haneul-move-concepts/packages/custom-policies',
						],
					},
					'concepts/haneul-move-concepts/conventions',
				],
			},
			{
				type: 'category',
				label: 'Transactions',
				link: {
					type: 'doc',
					id: 'concepts/transactions',
				},
				items: [
					'concepts/transactions/prog-txn-blocks',
					'concepts/transactions/sponsored-transactions',
					'concepts/transactions/gas-smashing',
				],
			},
		],
	},
	{
		type: 'category',
		label: 'Cryptography',
		link: {
			type: 'doc',
			id: 'concepts/cryptography',
		},
		items: [
			{
				type: 'category',
				label: 'Transaction Authentication',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/transaction-auth',
				},
				items: [
					'concepts/cryptography/transaction-auth/keys-addresses',
					'concepts/cryptography/transaction-auth/signatures',
					'concepts/cryptography/transaction-auth/multisig',
					'concepts/cryptography/transaction-auth/offline-signing',
					'concepts/cryptography/transaction-auth/intent-signing',
				],
			},
			'concepts/cryptography/zklogin',
			'concepts/cryptography/system/checkpoint-verification',
			/*{
				type: 'category',
				label: 'System',
				link: {
					type: 'doc',
					id: 'concepts/cryptography/system',
				},
				items: [
					'concepts/cryptography/system/validator-signatures',
					'concepts/cryptography/system/intents-for-validation',
					'concepts/cryptography/system/checkpoint-verification',
				],
			},*/
		],
	},
	{
		type: 'category',
		label: 'Haneul Architecture',
		link: {
			type: 'doc',
			id: 'concepts/haneul-architecture',
		},
		items: [
			'concepts/haneul-architecture/high-level',
			'concepts/haneul-architecture/haneul-storage',
			'concepts/haneul-architecture/haneul-security',
			'concepts/haneul-architecture/transaction-lifecycle',
			'concepts/haneul-architecture/consensus',
			'concepts/haneul-architecture/indexer-functions',
			'concepts/haneul-architecture/epochs',
			'concepts/haneul-architecture/protocol-upgrades',
			'concepts/haneul-architecture/data-management-things',
			'concepts/haneul-architecture/staking-rewards',
		],
	},
	{
		type: 'category',
		label: 'Tokenomics',
		link: {
			type: 'doc',
			id: 'concepts/tokenomics',
		},
		items: [
			'concepts/tokenomics/proof-of-stake',
			'concepts/tokenomics/validators-staking',
			'concepts/tokenomics/staking-unstaking',
			'concepts/tokenomics/haneul-coin',
			'concepts/tokenomics/haneul-bridging',
			'concepts/tokenomics/storage-fund',
			'concepts/tokenomics/gas-pricing',
			'concepts/tokenomics/gas-in-haneul',
		],
	},
	'concepts/research-papers',
];
module.exports = concepts;
