// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const concepts = [
	'concepts',
	'concepts/haneul-for-ethereum',
	'concepts/haneul-for-solana',
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
		items: [
			'concepts/transactions/transaction-lifecycle',
			'concepts/transactions/inputs-and-results',
			'concepts/transactions/gas-smashing',
			'concepts/transactions/transaction-auth',
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
	'concepts/coin-mgt',

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
			'concepts/data-access/graphql-rpc',
			'concepts/data-access/archival-store',
			{
				type: 'category',
				label: 'Custom Indexers',
				link: {
					type: 'doc',
					id: 'concepts/data-access/custom-indexers',
				},
				items: [
					'concepts/data-access/pipeline-architecture',
					'concepts/data-access/indexer-data-integration',
					'concepts/data-access/indexer-runtime-perf',
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
			'concepts/cryptography/zklogin',
			'concepts/cryptography/passkeys',
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
