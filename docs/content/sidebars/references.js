// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const references = [
	{
		type: 'doc',
		label: 'References',
		id: 'references',
	},
	{
		type: 'link',
		label: 'Haneul Framework (GitHub)',
		href: 'https://github.com/GeunhwaJeong/haneul/tree/main/crates/haneul-framework/docs',
	},
	{
		type: 'category',
		label: 'Haneul API',
		link: {
			type: 'doc',
			id: 'references/haneul-api',
		},
		items: [
			'references/haneul-api/beta-graph-ql',
			{
				type: 'link',
				label: 'API Reference',
				href: '/haneul-api-ref',
			},
			'references/haneul-api/rpc-best-practices',
		],
	},
	{
		type: 'category',
		label: 'Haneul CLI',
		link: {
			type: 'doc',
			id: 'references/cli',
		},
		items: [
			'references/cli/client',
			'references/cli/console',
			'references/cli/keytool',
			'references/cli/move',
			'references/cli/validator',
		],
	},
	{
		type: 'category',
		label: 'Haneul SDKs',
		link: {
			type: 'doc',
			id: 'references/haneul-sdks',
		},
		items: [
			{
				type: 'link',
				label: 'Haneul TypeScript SDK Site',
				href: 'https://haneul-typescript-docs.vercel.app/typescript',
			},
			'references/rust-sdk',
		],
	},
	{
		type: 'link',
		label: 'dApp Kit Site',
		href: 'https://haneul-typescript-docs.vercel.app/dapp-kit',
	},
	{
		type: 'category',
		label: 'Move',
		link: {
			type: 'doc',
			id: 'references/haneul-move',
		},
		items: [
			'references/move/move-toml',
			'references/move/move-lock',
			{
				type: 'link',
				label: 'Move Language (GitHub)',
				href: 'https://github.com/move-language/move/blob/main/language/documentation/book/src/introduction.md',
			},
		],
	},
	'references/haneul-glossary',
	{
		type: 'category',
		label: 'Contribute',
		link: {
			type: 'doc',
			id: 'references/contribute/contribution-process',
		},
		items: [
			'references/contribute/contribution-process',
			'references/contribute/contribute-to-haneul-repos',
			{
				type: 'link',
				label: 'Submit a SIP',
				href: 'https://sips.haneul.io',
			},
			'references/contribute/localize-haneul-docs',
			'references/contribute/code-of-conduct',
			'references/contribute/style-guide',
		],
	},
];

module.exports = references;
