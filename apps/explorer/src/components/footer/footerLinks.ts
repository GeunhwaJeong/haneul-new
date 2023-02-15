// Copyright (c) Mysten Labs, Inc.
// SPDX-License-Identifier: Apache-2.0

type FooterItem = {
    category: string;
    items: { title: string; href: string }[];
};
export type FooterItems = FooterItem[];
export const footerLinks = [
    {
        category: 'Read',
        items: [
            { title: 'Blog', href: 'https://medium.com/haneullabs-labs' },
            {
                title: 'Whitepaper',
                href: 'https://github.com/GeunhwaJeong/haneul/blob/main/doc/paper/haneul.pdf',
            },
        ],
    },
    {
        category: 'Build',
        items: [
            {
                title: 'Docs',
                href: 'https://docs.haneul.io/',
            },
            {
                title: 'GitHub',
                href: 'https://github.com/GeunhwaJeong',
            },
            {
                title: 'Discord',
                href: 'https://discord.gg/haneul',
            },
        ],
    },

    {
        category: 'Follow',
        items: [
            { title: 'Press', href: 'https://haneul-labs.com/#community' },
            {
                title: 'Twitter',
                href: 'https://twitter.com/HaneulNetwork',
            },
            {
                title: 'LinkedIn',
                href: 'https://www.linkedin.com/company/haneullabs-labs/',
            },
        ],
    },
    {
        category: 'Legal',
        items: [
            {
                title: 'Terms & Conditions',
                href: 'https://haneul-labs.com/legal?content=terms',
            },
            {
                title: 'Privacy Policy',
                href: 'https://haneul-labs.com/legal?content=privacy',
            },
        ],
    },
];
