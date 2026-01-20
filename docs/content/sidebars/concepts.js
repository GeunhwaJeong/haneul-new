// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const concepts = [
	'concepts',
	'concepts/haneul-for-ethereum',
	{
		type: 'category',
		label: 'Architecture',
		link: {
			type: 'doc',
			id: 'concepts/architecture',
		},
		items: [
			'concepts/haneul-architecture/networks',
			'concepts/haneul-architecture/haneul-storage',
			'concepts/haneul-architecture/consensus',
			'concepts/haneul-architecture/epochs',
			'concepts/haneul-architecture/haneul-security',
			'concepts/haneul-architecture/protocol-upgrades',
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
			'concepts/transactions/transaction-lifecycle',
			'concepts/transactions/prog-txn-blocks',
			'concepts/transactions/sponsored-transactions',
			'concepts/transactions/gas-smashing',
			'concepts/transactions/coin-mgt',
			{
				type: 'category',
				label: 'Transaction Authentication',
				link: {
					type: 'doc',
					id: 'concepts/transactions/transaction-auth',
				},
				items: [
					'concepts/transactions/transaction-auth/multisig',
					'concepts/transactions/transaction-auth/offline-signing',
					'concepts/transactions/transaction-auth/intent-signing',
				],
			},
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
			'concepts/tokenomics/staking-unstaking',
			'concepts/tokenomics/haneul-bridging',
			'concepts/tokenomics/gas-in-haneul',
		],
	},
	{
		type: 'category',
		label: 'Move',
		link: {
			type: 'doc',
			id: 'concepts/haneul-move-concepts',
		},
		items: [
			'concepts/haneul-move-concepts/packages',
			'concepts/haneul-move-concepts/conventions',
			'concepts/haneul-move-concepts/move-2024-migration',
		],
	},
	{
		type: 'category',
		label: 'Accessing Data',
		link: {
			type: 'doc',
			id: 'concepts/data-access/data-serving',
		},
		items: [
			'concepts/data-access/grpc',
			'concepts/data-access/graphql-indexer',
			'concepts/data-access/graphql-rpc',
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
			'concepts/cryptography/zklogin',
			'concepts/cryptography/passkeys',
			'concepts/cryptography/nautilus/nautilus-design',
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
				],
			},*/
		],
	},
	'concepts/gaming',
	'concepts/research-papers',
];
export default concepts;
