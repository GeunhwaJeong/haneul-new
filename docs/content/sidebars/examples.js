// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const examples = [
	{
		type: 'doc',
		id: 'examples',
		label: 'Examples',
	},
	{
		type: 'category',
		label: 'Move Basics',
		link: {
			type: 'doc',
			id: 'haneul-examples/movetoml',
		},
		items: [
			'haneul-examples/movetoml',
			'haneul-examples/init',
			'haneul-examples/entry-functions',
			'haneul-examples/strings',
			'haneul-examples/shared-objects',
			'haneul-examples/transferring-objects',
			'haneul-examples/custom-transfer',
			'haneul-examples/events',
			'haneul-examples/otw',
			'haneul-examples/publisher',
			'haneul-examples/object-display',
		],
	},
	{
		type: 'category',
		label: 'Patterns',
		link: {
			type: 'doc',
			id: 'haneul-examples/capability',
		},
		items: [
			'haneul-examples/capability',
			'haneul-examples/witness',
			'haneul-examples/transferrable-witness',
			'haneul-examples/hot-potato',
			'haneul-examples/id-pointer',
		],
	},
	{
		type: 'category',
		label: 'Samples',
		link: {
			type: 'doc',
			id: 'haneul-examples/create-an-nft',
		},
		items: ['haneul-examples/create-an-nft', 'haneul-examples/create-a-coin'],
	},
	'haneul-examples/additional-resources',
];

module.exports = examples;
