// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

import { SocialDiscord24, SocialLinkedin24, SocialTwitter24 } from '@haneullabs/icons';
import { type ReactNode } from 'react';

type FooterItem = {
	category: string;
	items: { title: string; children: ReactNode; href: string }[];
};
export type FooterItems = FooterItem[];

function FooterIcon({ children }: { children: ReactNode }) {
	return <div className="flex items-center text-steel-darker">{children}</div>;
}

export const footerLinks = [
	{ title: 'FAQ', href: 'https://docs.haneul-labs.com/explorer/faq' },
	{ title: 'Blog', href: 'https://medium.com/haneullabs-labs' },
	{
		title: 'Whitepaper',
		href: 'https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf',
	},
	{
		title: 'Docs',
		href: 'https://docs.haneul-labs.com/explorer',
	},
	{
		title: 'GitHub',
		href: 'https://github.com/GeunhwaJeong',
	},
	{ title: 'Press', href: 'https://haneul-labs.com/#community' },
];

export const socialLinks = [
	{
		children: (
			<FooterIcon>
				<SocialDiscord24 />
			</FooterIcon>
		),
		href: 'https://discord.gg/BK6WFhud',
	},
	{
		children: (
			<FooterIcon>
				<SocialTwitter24 />
			</FooterIcon>
		),
		href: 'https://twitter.com/Haneul_Labs',
	},
	{
		children: (
			<FooterIcon>
				<SocialLinkedin24 />
			</FooterIcon>
		),
		href: 'https://www.linkedin.com/company/haneullabs-labs/',
	},
];

export const legalLinks = [
	{
		title: 'Terms & Conditions',
		href: 'https://haneul-labs.com/legal#termsofservice',
	},
	{
		title: 'Privacy Policy',
		href: 'https://haneul-labs.com/legal#privacypolicy',
	},
];
