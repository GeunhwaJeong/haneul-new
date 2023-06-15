// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { DocsThemeConfig } from 'nextra-theme-docs';

const config: DocsThemeConfig = {
	logo: <span>Haneul Wallet Kit</span>,
	project: {
		link: 'https://github.com/GeunhwaJeong/haneul/tree/main/sdk/wallet-adapter',
	},
	chat: {
		link: 'https://discord.com/invite/Haneul',
	},
	docsRepositoryBase: 'https://github.com/GeunhwaJeong/haneul/tree/main/sdk/wallet-adapter',
	footer: {
		text: 'Copyright © 2023, Haneul Labs, Inc.',
	},
	useNextSeoProps() {
		return {
			titleTemplate: '%s – Haneul Wallet Kit',
		};
	},
};

export default config;
