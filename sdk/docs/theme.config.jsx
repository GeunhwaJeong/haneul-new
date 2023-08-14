// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

const config = {
	logo: <span>Haneul TypeScript Docs</span>,
	project: {
		link: 'https://github.com/GeunhwaJeong/haneul/tree/main/sdk/',
	},
	chat: {
		link: 'https://discord.com/invite/Haneul',
	},
	docsRepositoryBase: 'https://github.com/GeunhwaJeong/haneul/tree/main/sdk/docs/pages',
	footer: {
		text: 'Copyright © 2023, Haneul Labs, Inc.',
	},
	useNextSeoProps() {
		return {
			titleTemplate: '%s',
		};
	},
};

export default config;
