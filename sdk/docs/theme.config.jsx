// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useRouter } from 'next/router';

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
	head: (
		<>
			<meta name="google-site-verification" content="vRwZ3B3JxeegVwxa01XL4edDasP3pT8jPxEiBqqEaqw" />
			<meta httpEquiv="Content-Language" content="en" />
		</>
	),
	useNextSeoProps() {
		const { asPath } = useRouter();

		return {
			titleTemplate: asPath !== '/' ? '%s | Haneul TypeScript Docs' : 'Haneul TypeScript Docs',
			description:
				'Haneul TypeScript Documentation. Discover the power of Haneul through examples, guides, and concepts.',
			openGraph: {
				title: 'Haneul TypeScript Docs',
				description:
					'Haneul TypeScript Documentation. Discover the power of Haneul through examples, guides, and concepts.',
				site_name: 'Haneul TypeScript Docs',
			},
			additionalMetaTags: [{ content: 'Haneul TypeScript Docs', name: 'apple-mobile-web-app-title' }],
			twitter: {
				card: 'summary_large_image',
				site: '@Haneul_Labs',
			},
		};
	},
};

export default config;
